// Dense factorizations of zluda_solver_agx (cuSOLVER dense LU, Cholesky and
// QR), f32 values (the build adds f64 copies), i32 pivots and info, for every
// AGX generation like sgemm.mlir. Matrices are column-major with a leading
// dimension unless a kernel takes element strides (row, column). The host
// runs the factorizations one column at a time: a few launches per column.

gpu.module @agx_solver {
  func.func private @agx_sv_ld(%m0: memref<1xf32, 1>, %k: index) -> f32 {
    %big = arith.constant 4611686018427387904 : index
    %m = memref.reinterpret_cast %m0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %v = memref.load %m[%k] : memref<?xf32, strided<[1]>, 1>
    return %v : f32
  }
  func.func private @agx_sv_st(%m0: memref<1xf32, 1>, %k: index, %v: f32) {
    %big = arith.constant 4611686018427387904 : index
    %m = memref.reinterpret_cast %m0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    memref.store %v, %m[%k] : memref<?xf32, strided<[1]>, 1>
    return
  }
  func.func private @agx_sv_ldi(%m0: memref<1xi32, 1>, %k: index) -> i32 {
    %big = arith.constant 4611686018427387904 : index
    %m = memref.reinterpret_cast %m0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xi32, 1> to memref<?xi32, strided<[1]>, 1>
    %v = memref.load %m[%k] : memref<?xi32, strided<[1]>, 1>
    return %v : i32
  }
  func.func private @agx_sv_sti(%m0: memref<1xi32, 1>, %k: index, %v: i32) {
    %big = arith.constant 4611686018427387904 : index
    %m = memref.reinterpret_cast %m0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xi32, 1> to memref<?xi32, strided<[1]>, 1>
    memref.store %v, %m[%k] : memref<?xi32, strided<[1]>, 1>
    return
  }
  // The offset of element (i, j) with strides (rs, cs).
  func.func private @agx_sv_at(%i: index, %j: index, %rs: index, %cs: index) -> index {
    %a = arith.muli %i, %rs : index
    %b = arith.muli %j, %cs : index
    %o = arith.addi %a, %b : index
    return %o : index
  }
  // The pivot row of agx_lu_swap (ipiv is one-based).
  func.func private @agx_sv_piv(%ipiv0: memref<1xi32, 1>, %k: index) -> index {
    %one = arith.constant 1 : i32
    %p1 = func.call @agx_sv_ldi(%ipiv0, %k) : (memref<1xi32, 1>, index) -> i32
    %p0 = arith.subi %p1, %one : i32
    %p = arith.index_cast %p0 : i32 to index
    return %p : index
  }

  // info[0] = v. One thread.
  gpu.func @agx_sv_set_info(%info0: memref<1xi32, 1>, %v: i32) kernel {
    %i0 = arith.constant 0 : index
    %t = gpu.global_id x
    %first = arith.cmpi eq, %t, %i0 : index
    scf.if %first {
      func.call @agx_sv_sti(%info0, %i0, %v) : (memref<1xi32, 1>, index, i32) -> ()
    }
    gpu.return
  }

  // ---- LU with partial pivoting (getrf), column k of an m x n matrix

  // ipiv[k] = 1 + the row of the largest |a[i, k]|, i >= k; info = k + 1 for
  // the first zero pivot (info is cleared at k 0). One thread.
  gpu.func @agx_lu_pivot(%a0: memref<1xf32, 1>, %m_: i64, %k_: i64, %lda_: i64,
                         %ipiv0: memref<1xi32, 1>, %info0: memref<1xi32, 1>) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %z = arith.constant 0 : i32
    %f0 = arith.constant 0.0 : f32
    %m = arith.index_cast %m_ : i64 to index
    %k = arith.index_cast %k_ : i64 to index
    %lda = arith.index_cast %lda_ : i64 to index
    %t = gpu.global_id x
    %first = arith.cmpi eq, %t, %i0 : index
    scf.if %first {
      %k0 = arith.cmpi eq, %k, %i0 : index
      scf.if %k0 {
        func.call @agx_sv_sti(%info0, %i0, %z) : (memref<1xi32, 1>, index, i32) -> ()
      }
      %col = arith.muli %k, %lda : index
      %o = arith.addi %col, %k : index
      %v0 = func.call @agx_sv_ld(%a0, %o) : (memref<1xf32, 1>, index) -> f32
      %a0v = math.absf %v0 : f32
      %k1 = arith.addi %k, %i1 : index
      %best:2 = scf.for %i = %k1 to %m step %i1 iter_args(%bi = %k, %bv = %a0v) -> (index, f32) {
        %oi = arith.addi %col, %i : index
        %v = func.call @agx_sv_ld(%a0, %oi) : (memref<1xf32, 1>, index) -> f32
        %av = math.absf %v : f32
        %gt = arith.cmpf ogt, %av, %bv : f32
        %ni = arith.select %gt, %i, %bi : index
        %nv = arith.select %gt, %av, %bv : f32
        scf.yield %ni, %nv : index, f32
      }
      %b1 = arith.addi %best#0, %i1 : index
      %b1i = arith.index_cast %b1 : index to i32
      func.call @agx_sv_sti(%ipiv0, %k, %b1i) : (memref<1xi32, 1>, index, i32) -> ()
      %zero = arith.cmpf oeq, %best#1, %f0 : f32
      %old = func.call @agx_sv_ldi(%info0, %i0) : (memref<1xi32, 1>, index) -> i32
      %none = arith.cmpi eq, %old, %z : i32
      %set = arith.andi %zero, %none : i1
      scf.if %set {
        %ki = arith.index_cast %k1 : index to i32
        func.call @agx_sv_sti(%info0, %i0, %ki) : (memref<1xi32, 1>, index, i32) -> ()
      }
    }
    gpu.return
  }

  // Swaps rows k and ipiv[k] - 1 of the n columns (one thread per column).
  gpu.func @agx_lu_swap(%a0: memref<1xf32, 1>, %n_: i64, %lda_: i64, %k_: i64, %ipiv0: memref<1xi32, 1>) kernel {
    %n = arith.index_cast %n_ : i64 to index
    %lda = arith.index_cast %lda_ : i64 to index
    %k = arith.index_cast %k_ : i64 to index
    %j = gpu.global_id x
    %live = arith.cmpi ult, %j, %n : index
    scf.if %live {
      %p = func.call @agx_sv_piv(%ipiv0, %k) : (memref<1xi32, 1>, index) -> index
      %diff = arith.cmpi ne, %p, %k : index
      scf.if %diff {
        %col = arith.muli %j, %lda : index
        %ok = arith.addi %col, %k : index
        %op = arith.addi %col, %p : index
        %vk = func.call @agx_sv_ld(%a0, %ok) : (memref<1xf32, 1>, index) -> f32
        %vp = func.call @agx_sv_ld(%a0, %op) : (memref<1xf32, 1>, index) -> f32
        func.call @agx_sv_st(%a0, %ok, %vp) : (memref<1xf32, 1>, index, f32) -> ()
        func.call @agx_sv_st(%a0, %op, %vk) : (memref<1xf32, 1>, index, f32) -> ()
      }
    }
    gpu.return
  }

  // a[k + 1 + i, k] /= a[k, k] unless the pivot is 0 (one thread per row).
  gpu.func @agx_lu_scale(%a0: memref<1xf32, 1>, %m_: i64, %lda_: i64, %k_: i64) kernel {
    %i1 = arith.constant 1 : index
    %f0 = arith.constant 0.0 : f32
    %m = arith.index_cast %m_ : i64 to index
    %lda = arith.index_cast %lda_ : i64 to index
    %k = arith.index_cast %k_ : i64 to index
    %t = gpu.global_id x
    %k1 = arith.addi %k, %i1 : index
    %r = arith.addi %k1, %t : index
    %live = arith.cmpi ult, %r, %m : index
    scf.if %live {
      %col = arith.muli %k, %lda : index
      %od = arith.addi %col, %k : index
      %piv = func.call @agx_sv_ld(%a0, %od) : (memref<1xf32, 1>, index) -> f32
      %nz = arith.cmpf une, %piv, %f0 : f32
      scf.if %nz {
        %o = arith.addi %col, %r : index
        %v = func.call @agx_sv_ld(%a0, %o) : (memref<1xf32, 1>, index) -> f32
        %q = arith.divf %v, %piv : f32
        func.call @agx_sv_st(%a0, %o, %q) : (memref<1xf32, 1>, index, f32) -> ()
      }
    }
    gpu.return
  }

  // a[r, c] -= a[r, k] a[k, c] for k < r < m and k < c < n: launch an
  // (m - k - 1) x (n - k - 1) grid.
  gpu.func @agx_lu_update(%a0: memref<1xf32, 1>, %m_: i64, %n_: i64, %lda_: i64, %k_: i64) kernel {
    %i1 = arith.constant 1 : index
    %m = arith.index_cast %m_ : i64 to index
    %n = arith.index_cast %n_ : i64 to index
    %lda = arith.index_cast %lda_ : i64 to index
    %k = arith.index_cast %k_ : i64 to index
    %ti = gpu.global_id x
    %tj = gpu.global_id y
    %k1 = arith.addi %k, %i1 : index
    %r = arith.addi %k1, %ti : index
    %c = arith.addi %k1, %tj : index
    %in_r = arith.cmpi ult, %r, %m : index
    %in_c = arith.cmpi ult, %c, %n : index
    %live = arith.andi %in_r, %in_c : i1
    scf.if %live {
      %ck = arith.muli %k, %lda : index
      %cc = arith.muli %c, %lda : index
      %olk = arith.addi %ck, %r : index
      %oku = arith.addi %cc, %k : index
      %o = arith.addi %cc, %r : index
      %l = func.call @agx_sv_ld(%a0, %olk) : (memref<1xf32, 1>, index) -> f32
      %u = func.call @agx_sv_ld(%a0, %oku) : (memref<1xf32, 1>, index) -> f32
      %v = func.call @agx_sv_ld(%a0, %o) : (memref<1xf32, 1>, index) -> f32
      %nl = arith.negf %l : f32
      %nv = math.fma %nl, %u, %v : f32
      func.call @agx_sv_st(%a0, %o, %nv) : (memref<1xf32, 1>, index, f32) -> ()
    }
    gpu.return
  }

  // Solves op(T) x = b in place in b[boff + i], i < n, for the triangle of T
  // (lower or upper) in a with leading dimension lda: op(T)(i, j) is
  // T(j, i) with trans. A unit diagonal is not read.
  func.func private @agx_sv_trsv(%a0: memref<1xf32, 1>, %lda: index, %b0: memref<1xf32, 1>, %boff: index,
                                 %n: index, %lower: i1, %unit: i1, %trans: i1) {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    // op(T) is lower: forward substitution
    %fwd = arith.xori %lower, %trans : i1
    %n_1 = arith.subi %n, %i1 : index
    scf.for %s = %i0 to %n step %i1 {
      %back = arith.subi %n_1, %s : index
      %i = arith.select %fwd, %s, %back : index
      %i_1 = arith.addi %i, %i1 : index
      %lo = arith.select %fwd, %i0, %i_1 : index
      %hi = arith.select %fwd, %i, %n : index
      %bo = arith.addi %boff, %i : index
      %bi = func.call @agx_sv_ld(%b0, %bo) : (memref<1xf32, 1>, index) -> f32
      %sum = scf.for %j = %lo to %hi step %i1 iter_args(%acc = %bi) -> f32 {
        %r = arith.select %trans, %j, %i : index
        %c = arith.select %trans, %i, %j : index
        %o = func.call @agx_sv_at(%r, %c, %i1, %lda) : (index, index, index, index) -> index
        %t = func.call @agx_sv_ld(%a0, %o) : (memref<1xf32, 1>, index) -> f32
        %xo = arith.addi %boff, %j : index
        %x = func.call @agx_sv_ld(%b0, %xo) : (memref<1xf32, 1>, index) -> f32
        %nt = arith.negf %t : f32
        %acc1 = math.fma %nt, %x, %acc : f32
        scf.yield %acc1 : f32
      }
      %xi = scf.if %unit -> f32 {
        scf.yield %sum : f32
      } else {
        %od = func.call @agx_sv_at(%i, %i, %i1, %lda) : (index, index, index, index) -> index
        %d = func.call @agx_sv_ld(%a0, %od) : (memref<1xf32, 1>, index) -> f32
        %q = arith.divf %sum, %d : f32
        scf.yield %q : f32
      }
      func.call @agx_sv_st(%b0, %bo, %xi) : (memref<1xf32, 1>, index, f32) -> ()
    }
    return
  }

  // Applies the row swaps of ipiv to the column at boff, in order or in
  // reverse.
  func.func private @agx_sv_pivots(%ipiv0: memref<1xi32, 1>, %b0: memref<1xf32, 1>, %boff: index, %n: index, %reverse: i1) {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %n_1 = arith.subi %n, %i1 : index
    scf.for %s = %i0 to %n step %i1 {
      %back = arith.subi %n_1, %s : index
      %k = arith.select %reverse, %back, %s : index
      %p = func.call @agx_sv_piv(%ipiv0, %k) : (memref<1xi32, 1>, index) -> index
      %ok = arith.addi %boff, %k : index
      %op = arith.addi %boff, %p : index
      %vk = func.call @agx_sv_ld(%b0, %ok) : (memref<1xf32, 1>, index) -> f32
      %vp = func.call @agx_sv_ld(%b0, %op) : (memref<1xf32, 1>, index) -> f32
      func.call @agx_sv_st(%b0, %ok, %vp) : (memref<1xf32, 1>, index, f32) -> ()
      func.call @agx_sv_st(%b0, %op, %vk) : (memref<1xf32, 1>, index, f32) -> ()
    }
    return
  }

  // The row interchanges of getrf (ipiv) on the columns of B, in order, or in
  // reverse order (reverse 1); one thread per column. For the blocked getrs,
  // whose triangular solves run on BLAS trsm.
  gpu.func @agx_getrs_pivots(%ipiv0: memref<1xi32, 1>, %b0: memref<1xf32, 1>, %n_: i64, %nrhs_: i64,
                             %ldb_: i64, %reverse_: i64) kernel {
    %z = arith.constant 0 : i64
    %n = arith.index_cast %n_ : i64 to index
    %nrhs = arith.index_cast %nrhs_ : i64 to index
    %ldb = arith.index_cast %ldb_ : i64 to index
    %reverse = arith.cmpi ne, %reverse_, %z : i64
    %c = gpu.global_id x
    %live = arith.cmpi ult, %c, %nrhs : index
    scf.if %live {
      %boff = arith.muli %c, %ldb : index
      func.call @agx_sv_pivots(%ipiv0, %b0, %boff, %n, %reverse) : (memref<1xi32, 1>, memref<1xf32, 1>, index, index, i1) -> ()
    }
    gpu.return
  }

  // getrs: op(A) X = B with A = P L U from agx_lu_*, one thread per column
  // of B (n x nrhs, leading dimension ldb).
  gpu.func @agx_getrs(%a0: memref<1xf32, 1>, %n_: i64, %lda_: i64, %ipiv0: memref<1xi32, 1>,
                      %b0: memref<1xf32, 1>, %nrhs_: i64, %ldb_: i64, %trans_: i64) kernel {
    %z = arith.constant 0 : i64
    %true = arith.constant true
    %false = arith.constant false
    %n = arith.index_cast %n_ : i64 to index
    %lda = arith.index_cast %lda_ : i64 to index
    %nrhs = arith.index_cast %nrhs_ : i64 to index
    %ldb = arith.index_cast %ldb_ : i64 to index
    %trans = arith.cmpi ne, %trans_, %z : i64
    %c = gpu.global_id x
    %live = arith.cmpi ult, %c, %nrhs : index
    scf.if %live {
      %boff = arith.muli %c, %ldb : index
      scf.if %trans {
        func.call @agx_sv_trsv(%a0, %lda, %b0, %boff, %n, %false, %false, %true)
            : (memref<1xf32, 1>, index, memref<1xf32, 1>, index, index, i1, i1, i1) -> ()
        func.call @agx_sv_trsv(%a0, %lda, %b0, %boff, %n, %true, %true, %true)
            : (memref<1xf32, 1>, index, memref<1xf32, 1>, index, index, i1, i1, i1) -> ()
        func.call @agx_sv_pivots(%ipiv0, %b0, %boff, %n, %true) : (memref<1xi32, 1>, memref<1xf32, 1>, index, index, i1) -> ()
      } else {
        func.call @agx_sv_pivots(%ipiv0, %b0, %boff, %n, %false) : (memref<1xi32, 1>, memref<1xf32, 1>, index, index, i1) -> ()
        func.call @agx_sv_trsv(%a0, %lda, %b0, %boff, %n, %true, %true, %false)
            : (memref<1xf32, 1>, index, memref<1xf32, 1>, index, index, i1, i1, i1) -> ()
        func.call @agx_sv_trsv(%a0, %lda, %b0, %boff, %n, %false, %false, %false)
            : (memref<1xf32, 1>, index, memref<1xf32, 1>, index, index, i1, i1, i1) -> ()
      }
    }
    gpu.return
  }

  // ---- Cholesky (potrf), column k: L(i, j) is a[i rs + j cs] (rs 1 and
  // cs lda for the lower triangle, the other way round for the upper one,
  // whose U is L^T). Nothing happens once info is set.

  // a[k, k] = sqrt(a[k, k]), or info = k + 1 if it is not positive (info is
  // cleared at k 0). One thread.
  gpu.func @agx_chol_diag(%a0: memref<1xf32, 1>, %k_: i64, %rs_: i64, %cs_: i64, %info0: memref<1xi32, 1>) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %z = arith.constant 0 : i32
    %f0 = arith.constant 0.0 : f32
    %k = arith.index_cast %k_ : i64 to index
    %rs = arith.index_cast %rs_ : i64 to index
    %cs = arith.index_cast %cs_ : i64 to index
    %t = gpu.global_id x
    %first = arith.cmpi eq, %t, %i0 : index
    scf.if %first {
      %k0 = arith.cmpi eq, %k, %i0 : index
      scf.if %k0 {
        func.call @agx_sv_sti(%info0, %i0, %z) : (memref<1xi32, 1>, index, i32) -> ()
      }
      %old = func.call @agx_sv_ldi(%info0, %i0) : (memref<1xi32, 1>, index) -> i32
      %ok = arith.cmpi eq, %old, %z : i32
      scf.if %ok {
        %o = func.call @agx_sv_at(%k, %k, %rs, %cs) : (index, index, index, index) -> index
        %d = func.call @agx_sv_ld(%a0, %o) : (memref<1xf32, 1>, index) -> f32
        %pos = arith.cmpf ogt, %d, %f0 : f32
        scf.if %pos {
          %s = math.sqrt %d : f32
          func.call @agx_sv_st(%a0, %o, %s) : (memref<1xf32, 1>, index, f32) -> ()
        } else {
          %k1 = arith.addi %k, %i1 : index
          %ki = arith.index_cast %k1 : index to i32
          func.call @agx_sv_sti(%info0, %i0, %ki) : (memref<1xi32, 1>, index, i32) -> ()
        }
      }
    }
    gpu.return
  }

  // L(k + 1 + i, k) /= L(k, k) (one thread per row).
  gpu.func @agx_chol_scale(%a0: memref<1xf32, 1>, %n_: i64, %k_: i64, %rs_: i64, %cs_: i64, %info0: memref<1xi32, 1>) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %z = arith.constant 0 : i32
    %n = arith.index_cast %n_ : i64 to index
    %k = arith.index_cast %k_ : i64 to index
    %rs = arith.index_cast %rs_ : i64 to index
    %cs = arith.index_cast %cs_ : i64 to index
    %t = gpu.global_id x
    %k1 = arith.addi %k, %i1 : index
    %r = arith.addi %k1, %t : index
    %in = arith.cmpi ult, %r, %n : index
    %old = func.call @agx_sv_ldi(%info0, %i0) : (memref<1xi32, 1>, index) -> i32
    %ok = arith.cmpi eq, %old, %z : i32
    %live = arith.andi %in, %ok : i1
    scf.if %live {
      %od = func.call @agx_sv_at(%k, %k, %rs, %cs) : (index, index, index, index) -> index
      %d = func.call @agx_sv_ld(%a0, %od) : (memref<1xf32, 1>, index) -> f32
      %o = func.call @agx_sv_at(%r, %k, %rs, %cs) : (index, index, index, index) -> index
      %v = func.call @agx_sv_ld(%a0, %o) : (memref<1xf32, 1>, index) -> f32
      %q = arith.divf %v, %d : f32
      func.call @agx_sv_st(%a0, %o, %q) : (memref<1xf32, 1>, index, f32) -> ()
    }
    gpu.return
  }

  // L(r, c) -= L(r, k) L(c, k) for k < c <= r < n: launch an (n - k - 1)
  // square grid.
  gpu.func @agx_chol_update(%a0: memref<1xf32, 1>, %n_: i64, %k_: i64, %rs_: i64, %cs_: i64, %info0: memref<1xi32, 1>,
                            %ncol_: i64) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %z = arith.constant 0 : i32
    %n = arith.index_cast %n_ : i64 to index
    %k = arith.index_cast %k_ : i64 to index
    %rs = arith.index_cast %rs_ : i64 to index
    %cs = arith.index_cast %cs_ : i64 to index
    %ti = gpu.global_id x
    %tj = gpu.global_id y
    %k1 = arith.addi %k, %i1 : index
    %r = arith.addi %k1, %ti : index
    %c = arith.addi %k1, %tj : index
    %in_r = arith.cmpi ult, %r, %n : index
    %tri = arith.cmpi ule, %c, %r : index
    // columns below ncol only (a panel of the blocked factorization)
    %ncol = arith.index_cast %ncol_ : i64 to index
    %in_c = arith.cmpi ult, %c, %ncol : index
    %in0 = arith.andi %in_r, %tri : i1
    %in = arith.andi %in0, %in_c : i1
    %old = func.call @agx_sv_ldi(%info0, %i0) : (memref<1xi32, 1>, index) -> i32
    %ok = arith.cmpi eq, %old, %z : i32
    %live = arith.andi %in, %ok : i1
    scf.if %live {
      %orc = func.call @agx_sv_at(%r, %c, %rs, %cs) : (index, index, index, index) -> index
      %ork = func.call @agx_sv_at(%r, %k, %rs, %cs) : (index, index, index, index) -> index
      %ock = func.call @agx_sv_at(%c, %k, %rs, %cs) : (index, index, index, index) -> index
      %v = func.call @agx_sv_ld(%a0, %orc) : (memref<1xf32, 1>, index) -> f32
      %lr = func.call @agx_sv_ld(%a0, %ork) : (memref<1xf32, 1>, index) -> f32
      %lc = func.call @agx_sv_ld(%a0, %ock) : (memref<1xf32, 1>, index) -> f32
      %nl = arith.negf %lr : f32
      %nv = math.fma %nl, %lc, %v : f32
      func.call @agx_sv_st(%a0, %orc, %nv) : (memref<1xf32, 1>, index, f32) -> ()
    }
    gpu.return
  }

  // potrs: A X = B with A = L L^T (lower) or U^T U (upper) from agx_chol_*,
  // one thread per column of B.
  gpu.func @agx_potrs(%a0: memref<1xf32, 1>, %n_: i64, %lda_: i64, %b0: memref<1xf32, 1>,
                      %nrhs_: i64, %ldb_: i64, %upper_: i64) kernel {
    %z = arith.constant 0 : i64
    %true = arith.constant true
    %false = arith.constant false
    %n = arith.index_cast %n_ : i64 to index
    %lda = arith.index_cast %lda_ : i64 to index
    %nrhs = arith.index_cast %nrhs_ : i64 to index
    %ldb = arith.index_cast %ldb_ : i64 to index
    %upper = arith.cmpi ne, %upper_, %z : i64
    %lower = arith.xori %upper, %true : i1
    %c = gpu.global_id x
    %live = arith.cmpi ult, %c, %nrhs : index
    scf.if %live {
      %boff = arith.muli %c, %ldb : index
      // L y = b then L^T x = y; for the upper triangle U^T y = b then U x = y
      func.call @agx_sv_trsv(%a0, %lda, %b0, %boff, %n, %lower, %false, %upper)
          : (memref<1xf32, 1>, index, memref<1xf32, 1>, index, index, i1, i1, i1) -> ()
      func.call @agx_sv_trsv(%a0, %lda, %b0, %boff, %n, %lower, %false, %lower)
          : (memref<1xf32, 1>, index, memref<1xf32, 1>, index, index, i1, i1, i1) -> ()
    }
    gpu.return
  }

  // ---- Householder QR (geqrf), column k of an m x n matrix, LAPACK's
  // storage: v(k) = 1 implicit, v below the diagonal, beta on it, tau[k].

  // The reflector of column k (LAPACK's larfg without its rescaling). One
  // thread.
  gpu.func @agx_qr_house(%a0: memref<1xf32, 1>, %m_: i64, %k_: i64, %lda_: i64, %tau0: memref<1xf32, 1>) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %f0 = arith.constant 0.0 : f32
    %f1 = arith.constant 1.0 : f32
    %m = arith.index_cast %m_ : i64 to index
    %k = arith.index_cast %k_ : i64 to index
    %lda = arith.index_cast %lda_ : i64 to index
    %t = gpu.global_id x
    %first = arith.cmpi eq, %t, %i0 : index
    scf.if %first {
      %col = arith.muli %k, %lda : index
      %od = arith.addi %col, %k : index
      %alpha = func.call @agx_sv_ld(%a0, %od) : (memref<1xf32, 1>, index) -> f32
      %k1 = arith.addi %k, %i1 : index
      %ss = scf.for %i = %k1 to %m step %i1 iter_args(%s = %f0) -> f32 {
        %o = arith.addi %col, %i : index
        %v = func.call @agx_sv_ld(%a0, %o) : (memref<1xf32, 1>, index) -> f32
        %s1 = math.fma %v, %v, %s : f32
        scf.yield %s1 : f32
      }
      %zero = arith.cmpf oeq, %ss, %f0 : f32
      scf.if %zero {
        func.call @agx_sv_st(%tau0, %k, %f0) : (memref<1xf32, 1>, index, f32) -> ()
      } else {
        %all = math.fma %alpha, %alpha, %ss : f32
        %norm = math.sqrt %all : f32
        %neg = arith.negf %norm : f32
        %beta = math.copysign %neg, %alpha : f32
        %bma = arith.subf %beta, %alpha : f32
        %tau = arith.divf %bma, %beta : f32
        %amb = arith.subf %alpha, %beta : f32
        %scal = arith.divf %f1, %amb : f32
        scf.for %i = %k1 to %m step %i1 {
          %o = arith.addi %col, %i : index
          %v = func.call @agx_sv_ld(%a0, %o) : (memref<1xf32, 1>, index) -> f32
          %sv = arith.mulf %v, %scal : f32
          func.call @agx_sv_st(%a0, %o, %sv) : (memref<1xf32, 1>, index, f32) -> ()
        }
        func.call @agx_sv_st(%a0, %od, %beta) : (memref<1xf32, 1>, index, f32) -> ()
        func.call @agx_sv_st(%tau0, %k, %tau) : (memref<1xf32, 1>, index, f32) -> ()
      }
    }
    gpu.return
  }

  // x -= tau (v^T x) v for the reflector i (v from row i of column i of the
  // reflectors r, leading dimension ldr) and a vector x of length len with
  // elements at x[xoff + e es].
  func.func private @agx_sv_reflect(%r0: memref<1xf32, 1>, %ldr: index, %i: index, %tau: f32,
                                    %x0: memref<1xf32, 1>, %xoff: index, %es: index, %len: index) {
    %i1 = arith.constant 1 : index
    %col = arith.muli %i, %ldr : index
    %xi_o0 = arith.muli %i, %es : index
    %xi_o = arith.addi %xoff, %xi_o0 : index
    %xi = func.call @agx_sv_ld(%x0, %xi_o) : (memref<1xf32, 1>, index) -> f32
    %i_1 = arith.addi %i, %i1 : index
    %w0 = scf.for %e = %i_1 to %len step %i1 iter_args(%s = %xi) -> f32 {
      %vo = arith.addi %col, %e : index
      %v = func.call @agx_sv_ld(%r0, %vo) : (memref<1xf32, 1>, index) -> f32
      %xe0 = arith.muli %e, %es : index
      %xe = arith.addi %xoff, %xe0 : index
      %x = func.call @agx_sv_ld(%x0, %xe) : (memref<1xf32, 1>, index) -> f32
      %s1 = math.fma %v, %x, %s : f32
      scf.yield %s1 : f32
    }
    %w = arith.mulf %w0, %tau : f32
    %nxi = arith.subf %xi, %w : f32
    func.call @agx_sv_st(%x0, %xi_o, %nxi) : (memref<1xf32, 1>, index, f32) -> ()
    %nw = arith.negf %w : f32
    scf.for %e = %i_1 to %len step %i1 {
      %vo = arith.addi %col, %e : index
      %v = func.call @agx_sv_ld(%r0, %vo) : (memref<1xf32, 1>, index) -> f32
      %xe0 = arith.muli %e, %es : index
      %xe = arith.addi %xoff, %xe0 : index
      %x = func.call @agx_sv_ld(%x0, %xe) : (memref<1xf32, 1>, index) -> f32
      %nx = math.fma %nw, %v, %x : f32
      func.call @agx_sv_st(%x0, %xe, %nx) : (memref<1xf32, 1>, index, f32) -> ()
    }
    return
  }

  // Blocked geqrf (compact WY, Q = I - V T V^T for the reflectors j0 .. j0 +
  // jb - 1): V, the (m - j0) x jb matrix of their vectors (leading dimension
  // m - j0) with the implicit 1 on the diagonal and zeros above it. 2-D grid.
  gpu.func @agx_qr_v(%a0: memref<1xf32, 1>, %v0: memref<1xf32, 1>, %m_: i64, %j0_: i64, %jb_: i64, %lda_: i64) kernel {
    %f0 = arith.constant 0.0 : f32
    %f1 = arith.constant 1.0 : f32
    %m = arith.index_cast %m_ : i64 to index
    %j0 = arith.index_cast %j0_ : i64 to index
    %jb = arith.index_cast %jb_ : i64 to index
    %lda = arith.index_cast %lda_ : i64 to index
    %mv = arith.subi %m, %j0 : index
    %i = gpu.global_id x
    %c = gpu.global_id y
    %in_i = arith.cmpi ult, %i, %mv : index
    %in_c = arith.cmpi ult, %c, %jb : index
    %in = arith.andi %in_i, %in_c : i1
    scf.if %in {
      %below = arith.cmpi ugt, %i, %c : index
      %diag = arith.cmpi eq, %i, %c : index
      %v = scf.if %below -> f32 {
        %r = arith.addi %j0, %i : index
        %cc = arith.addi %j0, %c : index
        %col = arith.muli %cc, %lda : index
        %o = arith.addi %col, %r : index
        %x = func.call @agx_sv_ld(%a0, %o) : (memref<1xf32, 1>, index) -> f32
        scf.yield %x : f32
      } else {
        %d = arith.select %diag, %f1, %f0 : f32
        scf.yield %d : f32
      }
      %cm = arith.muli %c, %mv : index
      %ov = arith.addi %cm, %i : index
      func.call @agx_sv_st(%v0, %ov, %v) : (memref<1xf32, 1>, index, f32) -> ()
    }
    gpu.return
  }

  // T (jb x jb, leading dimension jb, zeros below the diagonal) of the
  // compact WY form from S = V^T V and tau[j0 ..]: T(i, i) = tau_i,
  // T(0 .. i, i) = -tau_i T(0 .. i, 0 .. i) S(0 .. i, i). One thread (jb^3 / 6
  // multiply-adds).
  gpu.func @agx_qr_t(%s0: memref<1xf32, 1>, %tau0: memref<1xf32, 1>, %t0: memref<1xf32, 1>, %j0_: i64, %jb_: i64) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %f0 = arith.constant 0.0 : f32
    %j0 = arith.index_cast %j0_ : i64 to index
    %jb = arith.index_cast %jb_ : i64 to index
    %t = gpu.global_id x
    %first = arith.cmpi eq, %t, %i0 : index
    scf.if %first {
      scf.for %i = %i0 to %jb step %i1 {
        %ti_at = arith.addi %j0, %i : index
        %ti = func.call @agx_sv_ld(%tau0, %ti_at) : (memref<1xf32, 1>, index) -> f32
        %nti = arith.negf %ti : f32
        %ci = arith.muli %i, %jb : index
        %oii = arith.addi %ci, %i : index
        func.call @agx_sv_st(%t0, %oii, %ti) : (memref<1xf32, 1>, index, f32) -> ()
        // rows above the diagonal (T(r, q) for q >= r only), then zeros below
        scf.for %r = %i0 to %i step %i1 {
          %w = scf.for %q = %r to %i step %i1 iter_args(%acc = %f0) -> f32 {
            %cq = arith.muli %q, %jb : index
            %orq = arith.addi %cq, %r : index
            %tq = func.call @agx_sv_ld(%t0, %orq) : (memref<1xf32, 1>, index) -> f32
            %oqi = arith.addi %ci, %q : index
            %sq = func.call @agx_sv_ld(%s0, %oqi) : (memref<1xf32, 1>, index) -> f32
            %a1 = math.fma %tq, %sq, %acc : f32
            scf.yield %a1 : f32
          }
          %v = arith.mulf %nti, %w : f32
          %ori = arith.addi %ci, %r : index
          func.call @agx_sv_st(%t0, %ori, %v) : (memref<1xf32, 1>, index, f32) -> ()
        }
        %i_1 = arith.addi %i, %i1 : index
        scf.for %r = %i_1 to %jb step %i1 {
          %ori = arith.addi %ci, %r : index
          func.call @agx_sv_st(%t0, %ori, %f0) : (memref<1xf32, 1>, index, f32) -> ()
        }
      }
    }
    gpu.return
  }

  // Applies the reflector of column k to the columns k + 1 + j < n (one
  // thread per column).
  gpu.func @agx_qr_apply(%a0: memref<1xf32, 1>, %m_: i64, %n_: i64, %k_: i64, %lda_: i64, %tau0: memref<1xf32, 1>) kernel {
    %i1 = arith.constant 1 : index
    %m = arith.index_cast %m_ : i64 to index
    %n = arith.index_cast %n_ : i64 to index
    %k = arith.index_cast %k_ : i64 to index
    %lda = arith.index_cast %lda_ : i64 to index
    %t = gpu.global_id x
    %k1 = arith.addi %k, %i1 : index
    %c = arith.addi %k1, %t : index
    %live = arith.cmpi ult, %c, %n : index
    scf.if %live {
      %tau = func.call @agx_sv_ld(%tau0, %k) : (memref<1xf32, 1>, index) -> f32
      %xoff = arith.muli %c, %lda : index
      func.call @agx_sv_reflect(%a0, %lda, %k, %tau, %a0, %xoff, %i1, %m)
          : (memref<1xf32, 1>, index, index, f32, memref<1xf32, 1>, index, index, index) -> ()
    }
    gpu.return
  }

  // ormqr: C (m x n, leading dimension ldc) = op(Q) C (left) or C op(Q)
  // (right) with Q = H(0) ... H(k - 1) from the reflectors r (leading
  // dimension ldr). One thread per column (left) or row (right) of C, which
  // applies the reflectors in order or in reverse.
  gpu.func @agx_ormqr(%r0: memref<1xf32, 1>, %ldr_: i64, %tau0: memref<1xf32, 1>, %c0: memref<1xf32, 1>,
                      %m_: i64, %n_: i64, %k_: i64, %ldc_: i64, %left_: i64, %trans_: i64) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %z = arith.constant 0 : i64
    %ldr = arith.index_cast %ldr_ : i64 to index
    %m = arith.index_cast %m_ : i64 to index
    %n = arith.index_cast %n_ : i64 to index
    %k = arith.index_cast %k_ : i64 to index
    %ldc = arith.index_cast %ldc_ : i64 to index
    %left = arith.cmpi ne, %left_, %z : i64
    %trans = arith.cmpi ne, %trans_, %z : i64
    // Q^T C and C Q apply H(0) first
    %fwd = arith.cmpi eq, %left, %trans : i1
    %count = arith.select %left, %n, %m : index
    %len = arith.select %left, %m, %n : index
    %es = arith.select %left, %i1, %ldc : index
    %vs = arith.select %left, %ldc, %i1 : index
    %t = gpu.global_id x
    %live = arith.cmpi ult, %t, %count : index
    scf.if %live {
      %xoff = arith.muli %t, %vs : index
      %k_1 = arith.subi %k, %i1 : index
      scf.for %s = %i0 to %k step %i1 {
        %back = arith.subi %k_1, %s : index
        %i = arith.select %fwd, %s, %back : index
        %tau = func.call @agx_sv_ld(%tau0, %i) : (memref<1xf32, 1>, index) -> f32
        func.call @agx_sv_reflect(%r0, %ldr, %i, %tau, %c0, %xoff, %es, %len)
            : (memref<1xf32, 1>, index, index, f32, memref<1xf32, 1>, index, index, index) -> ()
      }
    }
    gpu.return
  }

  // dst[i + j ldd] = src[i + j lds], or the identity when eye is not 0, for
  // i < m and j < n: an m x n grid.
  gpu.func @agx_sv_copy(%src0: memref<1xf32, 1>, %lds_: i64, %dst0: memref<1xf32, 1>, %ldd_: i64,
                        %m_: i64, %n_: i64, %eye_: i64) kernel {
    %z = arith.constant 0 : i64
    %f0 = arith.constant 0.0 : f32
    %f1 = arith.constant 1.0 : f32
    %lds = arith.index_cast %lds_ : i64 to index
    %ldd = arith.index_cast %ldd_ : i64 to index
    %m = arith.index_cast %m_ : i64 to index
    %n = arith.index_cast %n_ : i64 to index
    %eye = arith.cmpi ne, %eye_, %z : i64
    %i = gpu.global_id x
    %j = gpu.global_id y
    %in_i = arith.cmpi ult, %i, %m : index
    %in_j = arith.cmpi ult, %j, %n : index
    %live = arith.andi %in_i, %in_j : i1
    scf.if %live {
      %v = scf.if %eye -> f32 {
        %d = arith.cmpi eq, %i, %j : index
        %e = arith.select %d, %f1, %f0 : f32
        scf.yield %e : f32
      } else {
        %cs = arith.muli %j, %lds : index
        %os = arith.addi %cs, %i : index
        %s = func.call @agx_sv_ld(%src0, %os) : (memref<1xf32, 1>, index) -> f32
        scf.yield %s : f32
      }
      %cd = arith.muli %j, %ldd : index
      %od = arith.addi %cd, %i : index
      func.call @agx_sv_st(%dst0, %od, %v) : (memref<1xf32, 1>, index, f32) -> ()
    }
    gpu.return
  }

  // ---- one-sided Jacobi (Hestenes) for gesvd and syevd: rotations of
  // disjoint column pairs make the columns of A orthogonal; V collects them.

  // Round r of a sweep over the columns of A (m x n): pair t of the circle
  // schedule for N (n rounded up to even) columns, (N - 1, r) for t 0 and
  // ((r + t) mod (N - 1), (r - t) mod (N - 1)) otherwise, is rotated unless
  // |a_p . a_q| <= tol |a_p| |a_q|; the same rotation is applied to the vr
  // rows of V (0 for none). count[0] += 1 per rotation. One thread per pair.
  gpu.func @agx_jacobi_round(%a0: memref<1xf32, 1>, %m_: i64, %n_: i64, %lda_: i64, %v0: memref<1xf32, 1>,
                             %vr_: i64, %ldv_: i64, %r_: i64, %count0: memref<1xi32, 1>, %tol: f32) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %i2 = arith.constant 2 : index
    %f0 = arith.constant 0.0 : f32
    %f1 = arith.constant 1.0 : f32
    %f2 = arith.constant 2.0 : f32
    %one = arith.constant 1 : i32
    %m = arith.index_cast %m_ : i64 to index
    %n = arith.index_cast %n_ : i64 to index
    %lda = arith.index_cast %lda_ : i64 to index
    %vr = arith.index_cast %vr_ : i64 to index
    %ldv = arith.index_cast %ldv_ : i64 to index
    %r = arith.index_cast %r_ : i64 to index
    %odd = arith.remui %n, %i2 : index
    %nn = arith.addi %n, %odd : index
    %half = arith.divui %nn, %i2 : index
    %nn_1 = arith.subi %nn, %i1 : index
    %t = gpu.global_id x
    %live0 = arith.cmpi ult, %t, %half : index
    scf.if %live0 {
      %t0 = arith.cmpi eq, %t, %i0 : index
      %rt = arith.addi %r, %t : index
      %pa = arith.remui %rt, %nn_1 : index
      %rn = arith.addi %r, %nn_1 : index
      %rmt = arith.subi %rn, %t : index
      %qa = arith.remui %rmt, %nn_1 : index
      %p = arith.select %t0, %nn_1, %pa : index
      %q = arith.select %t0, %r, %qa : index
      %in_p = arith.cmpi ult, %p, %n : index
      %in_q = arith.cmpi ult, %q, %n : index
      %live = arith.andi %in_p, %in_q : i1
      scf.if %live {
        %cp = arith.muli %p, %lda : index
        %cq = arith.muli %q, %lda : index
        %sums:3 = scf.for %i = %i0 to %m step %i1 iter_args(%al = %f0, %be = %f0, %ga = %f0) -> (f32, f32, f32) {
          %op = arith.addi %cp, %i : index
          %oq = arith.addi %cq, %i : index
          %x = func.call @agx_sv_ld(%a0, %op) : (memref<1xf32, 1>, index) -> f32
          %y = func.call @agx_sv_ld(%a0, %oq) : (memref<1xf32, 1>, index) -> f32
          %al1 = math.fma %x, %x, %al : f32
          %be1 = math.fma %y, %y, %be : f32
          %ga1 = math.fma %x, %y, %ga : f32
          scf.yield %al1, %be1, %ga1 : f32, f32, f32
        }
        %ab = arith.mulf %sums#0, %sums#1 : f32
        %nab = math.sqrt %ab : f32
        %lim = arith.mulf %nab, %tol : f32
        %ag = math.absf %sums#2 : f32
        %rot = arith.cmpf ogt, %ag, %lim : f32
        scf.if %rot {
          // zeta = (beta - alpha) / (2 gamma), t = sign(zeta) / (|zeta| + sqrt(1 + zeta^2))
          %bma = arith.subf %sums#1, %sums#0 : f32
          %g2 = arith.mulf %sums#2, %f2 : f32
          %zeta = arith.divf %bma, %g2 : f32
          %az = math.absf %zeta : f32
          %z2 = math.fma %zeta, %zeta, %f1 : f32
          %sq = math.sqrt %z2 : f32
          %den = arith.addf %az, %sq : f32
          %tt0 = arith.divf %f1, %den : f32
          %tt = math.copysign %tt0, %zeta : f32
          %t2 = math.fma %tt, %tt, %f1 : f32
          %st2 = math.sqrt %t2 : f32
          %c = arith.divf %f1, %st2 : f32
          %s = arith.mulf %c, %tt : f32
          scf.for %i = %i0 to %m step %i1 {
            %op = arith.addi %cp, %i : index
            %oq = arith.addi %cq, %i : index
            func.call @agx_sv_rot(%a0, %op, %oq, %c, %s) : (memref<1xf32, 1>, index, index, f32, f32) -> ()
          }
          %vp = arith.muli %p, %ldv : index
          %vq = arith.muli %q, %ldv : index
          scf.for %i = %i0 to %vr step %i1 {
            %op = arith.addi %vp, %i : index
            %oq = arith.addi %vq, %i : index
            func.call @agx_sv_rot(%v0, %op, %oq, %c, %s) : (memref<1xf32, 1>, index, index, f32, f32) -> ()
          }
          %big2 = arith.constant 4611686018427387904 : index
          %cnt = memref.reinterpret_cast %count0 to offset: [0], sizes: [%big2], strides: [1]
              : memref<1xi32, 1> to memref<?xi32, strided<[1]>, 1>
          %old = memref.atomic_rmw addi %one, %cnt[%i0] : (i32, memref<?xi32, strided<[1]>, 1>) -> i32
        }
      }
    }
    gpu.return
  }

  // (x[p], x[q]) = (c x[p] - s x[q], s x[p] + c x[q]).
  func.func private @agx_sv_rot(%x0: memref<1xf32, 1>, %p: index, %q: index, %c: f32, %s: f32) {
    %x = func.call @agx_sv_ld(%x0, %p) : (memref<1xf32, 1>, index) -> f32
    %y = func.call @agx_sv_ld(%x0, %q) : (memref<1xf32, 1>, index) -> f32
    %sy = arith.mulf %s, %y : f32
    %nsy = arith.negf %sy : f32
    %nx = math.fma %c, %x, %nsy : f32
    %sx = arith.mulf %s, %x : f32
    %ny = math.fma %c, %y, %sx : f32
    func.call @agx_sv_st(%x0, %p, %nx) : (memref<1xf32, 1>, index, f32) -> ()
    func.call @agx_sv_st(%x0, %q, %ny) : (memref<1xf32, 1>, index, f32) -> ()
    return
  }

  // s[j] = |a_j| and a_j /= s[j] when it is not 0 (one thread per column).
  gpu.func @agx_jacobi_norms(%a0: memref<1xf32, 1>, %m_: i64, %n_: i64, %lda_: i64, %s0: memref<1xf32, 1>) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %f0 = arith.constant 0.0 : f32
    %m = arith.index_cast %m_ : i64 to index
    %n = arith.index_cast %n_ : i64 to index
    %lda = arith.index_cast %lda_ : i64 to index
    %j = gpu.global_id x
    %live = arith.cmpi ult, %j, %n : index
    scf.if %live {
      %col = arith.muli %j, %lda : index
      %ss = scf.for %i = %i0 to %m step %i1 iter_args(%acc = %f0) -> f32 {
        %o = arith.addi %col, %i : index
        %x = func.call @agx_sv_ld(%a0, %o) : (memref<1xf32, 1>, index) -> f32
        %acc1 = math.fma %x, %x, %acc : f32
        scf.yield %acc1 : f32
      }
      %nrm = math.sqrt %ss : f32
      func.call @agx_sv_st(%s0, %j, %nrm) : (memref<1xf32, 1>, index, f32) -> ()
      %nz = arith.cmpf ogt, %nrm, %f0 : f32
      scf.if %nz {
        scf.for %i = %i0 to %m step %i1 {
          %o = arith.addi %col, %i : index
          %x = func.call @agx_sv_ld(%a0, %o) : (memref<1xf32, 1>, index) -> f32
          %q = arith.divf %x, %nrm : f32
          func.call @agx_sv_st(%a0, %o, %q) : (memref<1xf32, 1>, index, f32) -> ()
        }
      }
    }
    gpu.return
  }

  // Sorts s[0 .. n) descending (ascending with asc not 0), swapping the
  // columns of U (ur rows) and V (vr rows) along. One thread.
  gpu.func @agx_jacobi_sort(%s0: memref<1xf32, 1>, %n_: i64, %u0: memref<1xf32, 1>, %ur_: i64, %ldu_: i64,
                            %v0: memref<1xf32, 1>, %vr_: i64, %ldv_: i64, %asc_: i64) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %z = arith.constant 0 : i64
    %n = arith.index_cast %n_ : i64 to index
    %ur = arith.index_cast %ur_ : i64 to index
    %ldu = arith.index_cast %ldu_ : i64 to index
    %vr = arith.index_cast %vr_ : i64 to index
    %ldv = arith.index_cast %ldv_ : i64 to index
    %asc = arith.cmpi ne, %asc_, %z : i64
    %t = gpu.global_id x
    %first = arith.cmpi eq, %t, %i0 : index
    scf.if %first {
      scf.for %i = %i0 to %n step %i1 {
        %si = func.call @agx_sv_ld(%s0, %i) : (memref<1xf32, 1>, index) -> f32
        %i_1 = arith.addi %i, %i1 : index
        %best:2 = scf.for %j = %i_1 to %n step %i1 iter_args(%bj = %i, %bv = %si) -> (index, f32) {
          %sj = func.call @agx_sv_ld(%s0, %j) : (memref<1xf32, 1>, index) -> f32
          %gt = arith.cmpf ogt, %sj, %bv : f32
          %lt = arith.cmpf olt, %sj, %bv : f32
          %better = arith.select %asc, %lt, %gt : i1
          %nj = arith.select %better, %j, %bj : index
          %nv = arith.select %better, %sj, %bv : f32
          scf.yield %nj, %nv : index, f32
        }
        %swap = arith.cmpi ne, %best#0, %i : index
        scf.if %swap {
          func.call @agx_sv_st(%s0, %best#0, %si) : (memref<1xf32, 1>, index, f32) -> ()
          func.call @agx_sv_st(%s0, %i, %best#1) : (memref<1xf32, 1>, index, f32) -> ()
          %ui = arith.muli %i, %ldu : index
          %ub = arith.muli %best#0, %ldu : index
          func.call @agx_sv_swapcols(%u0, %ui, %ub, %ur) : (memref<1xf32, 1>, index, index, index) -> ()
          %vi = arith.muli %i, %ldv : index
          %vb = arith.muli %best#0, %ldv : index
          func.call @agx_sv_swapcols(%v0, %vi, %vb, %vr) : (memref<1xf32, 1>, index, index, index) -> ()
        }
      }
    }
    gpu.return
  }

  func.func private @agx_sv_swapcols(%x0: memref<1xf32, 1>, %a: index, %b: index, %rows: index) {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    scf.for %i = %i0 to %rows step %i1 {
      %oa = arith.addi %a, %i : index
      %ob = arith.addi %b, %i : index
      %x = func.call @agx_sv_ld(%x0, %oa) : (memref<1xf32, 1>, index) -> f32
      %y = func.call @agx_sv_ld(%x0, %ob) : (memref<1xf32, 1>, index) -> f32
      func.call @agx_sv_st(%x0, %oa, %y) : (memref<1xf32, 1>, index, f32) -> ()
      func.call @agx_sv_st(%x0, %ob, %x) : (memref<1xf32, 1>, index, f32) -> ()
    }
    return
  }

  // Element (i, j) of the symmetric matrix whose upper (or lower) triangle
  // is stored in a.
  func.func private @agx_sv_sym(%a0: memref<1xf32, 1>, %lda: index, %i: index, %j: index, %upper: i1) -> f32 {
    %le = arith.cmpi ule, %i, %j : index
    %in = arith.cmpi eq, %le, %upper : i1
    %eq = arith.cmpi eq, %i, %j : index
    %direct = arith.ori %in, %eq : i1
    %r = arith.select %direct, %i, %j : index
    %c = arith.select %direct, %j, %i : index
    %cl = arith.muli %c, %lda : index
    %o = arith.addi %cl, %r : index
    %v = func.call @agx_sv_ld(%a0, %o) : (memref<1xf32, 1>, index) -> f32
    return %v : f32
  }

  // slot[0] = the Frobenius norm of the symmetric matrix. One thread.
  gpu.func @agx_eig_shift(%a0: memref<1xf32, 1>, %lda_: i64, %n_: i64, %upper_: i64, %slot0: memref<1xf32, 1>) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %z = arith.constant 0 : i64
    %f0 = arith.constant 0.0 : f32
    %lda = arith.index_cast %lda_ : i64 to index
    %n = arith.index_cast %n_ : i64 to index
    %upper = arith.cmpi ne, %upper_, %z : i64
    %t = gpu.global_id x
    %first = arith.cmpi eq, %t, %i0 : index
    scf.if %first {
      %ss = scf.for %j = %i0 to %n step %i1 iter_args(%acc = %f0) -> f32 {
        %acc2 = scf.for %i = %i0 to %n step %i1 iter_args(%acc1 = %acc) -> f32 {
          %x = func.call @agx_sv_sym(%a0, %lda, %i, %j, %upper) : (memref<1xf32, 1>, index, index, index, i1) -> f32
          %a = math.fma %x, %x, %acc1 : f32
          scf.yield %a : f32
        }
        scf.yield %acc2 : f32
      }
      %nrm = math.sqrt %ss : f32
      func.call @agx_sv_st(%slot0, %i0, %nrm) : (memref<1xf32, 1>, index, f32) -> ()
    }
    gpu.return
  }

  // W = the symmetric matrix + slot[0] I (so it is positive semidefinite and
  // its singular vectors are its eigenvectors) and V = I, both n x n with
  // leading dimension n: an n x n grid.
  gpu.func @agx_eig_prep(%a0: memref<1xf32, 1>, %lda_: i64, %n_: i64, %upper_: i64, %w0: memref<1xf32, 1>,
                         %v0: memref<1xf32, 1>, %slot0: memref<1xf32, 1>) kernel {
    %i0 = arith.constant 0 : index
    %z = arith.constant 0 : i64
    %f0 = arith.constant 0.0 : f32
    %f1 = arith.constant 1.0 : f32
    %lda = arith.index_cast %lda_ : i64 to index
    %n = arith.index_cast %n_ : i64 to index
    %upper = arith.cmpi ne, %upper_, %z : i64
    %i = gpu.global_id x
    %j = gpu.global_id y
    %in_i = arith.cmpi ult, %i, %n : index
    %in_j = arith.cmpi ult, %j, %n : index
    %live = arith.andi %in_i, %in_j : i1
    scf.if %live {
      %x = func.call @agx_sv_sym(%a0, %lda, %i, %j, %upper) : (memref<1xf32, 1>, index, index, index, i1) -> f32
      %d = arith.cmpi eq, %i, %j : index
      %shift = func.call @agx_sv_ld(%slot0, %i0) : (memref<1xf32, 1>, index) -> f32
      %sh = arith.select %d, %shift, %f0 : f32
      %wv = arith.addf %x, %sh : f32
      %e = arith.select %d, %f1, %f0 : f32
      %cl = arith.muli %j, %n : index
      %o = arith.addi %cl, %i : index
      func.call @agx_sv_st(%w0, %o, %wv) : (memref<1xf32, 1>, index, f32) -> ()
      func.call @agx_sv_st(%v0, %o, %e) : (memref<1xf32, 1>, index, f32) -> ()
    }
    gpu.return
  }

  // w[j] = v_j^T A v_j for the symmetric A and the columns of V (n x n,
  // leading dimension n): the eigenvalues. One thread per column.
  gpu.func @agx_eig_values(%a0: memref<1xf32, 1>, %lda_: i64, %n_: i64, %upper_: i64, %v0: memref<1xf32, 1>,
                           %w0: memref<1xf32, 1>) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %z = arith.constant 0 : i64
    %f0 = arith.constant 0.0 : f32
    %lda = arith.index_cast %lda_ : i64 to index
    %n = arith.index_cast %n_ : i64 to index
    %upper = arith.cmpi ne, %upper_, %z : i64
    %j = gpu.global_id x
    %live = arith.cmpi ult, %j, %n : index
    scf.if %live {
      %col = arith.muli %j, %n : index
      %q = scf.for %i = %i0 to %n step %i1 iter_args(%acc = %f0) -> f32 {
        %oi = arith.addi %col, %i : index
        %vi = func.call @agx_sv_ld(%v0, %oi) : (memref<1xf32, 1>, index) -> f32
        %av = scf.for %k = %i0 to %n step %i1 iter_args(%s = %f0) -> f32 {
          %ok = arith.addi %col, %k : index
          %vk = func.call @agx_sv_ld(%v0, %ok) : (memref<1xf32, 1>, index) -> f32
          %x = func.call @agx_sv_sym(%a0, %lda, %i, %k, %upper) : (memref<1xf32, 1>, index, index, index, i1) -> f32
          %s1 = math.fma %x, %vk, %s : f32
          scf.yield %s1 : f32
        }
        %acc1 = math.fma %vi, %av, %acc : f32
        scf.yield %acc1 : f32
      }
      func.call @agx_sv_st(%w0, %j, %q) : (memref<1xf32, 1>, index, f32) -> ()
    }
    gpu.return
  }

  // dst[j + i ldd] = src[i + j lds] for i < m and j < n (a transposed copy):
  // an m x n grid.
  gpu.func @agx_sv_transpose(%src0: memref<1xf32, 1>, %lds_: i64, %dst0: memref<1xf32, 1>, %ldd_: i64,
                             %m_: i64, %n_: i64) kernel {
    %lds = arith.index_cast %lds_ : i64 to index
    %ldd = arith.index_cast %ldd_ : i64 to index
    %m = arith.index_cast %m_ : i64 to index
    %n = arith.index_cast %n_ : i64 to index
    %i = gpu.global_id x
    %j = gpu.global_id y
    %in_i = arith.cmpi ult, %i, %m : index
    %in_j = arith.cmpi ult, %j, %n : index
    %live = arith.andi %in_i, %in_j : i1
    scf.if %live {
      %cs = arith.muli %j, %lds : index
      %os = arith.addi %cs, %i : index
      %x = func.call @agx_sv_ld(%src0, %os) : (memref<1xf32, 1>, index) -> f32
      %cd = arith.muli %i, %ldd : index
      %od = arith.addi %cd, %j : index
      func.call @agx_sv_st(%dst0, %od, %x) : (memref<1xf32, 1>, index, f32) -> ()
    }
    gpu.return
  }
}
