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
  gpu.func @agx_chol_update(%a0: memref<1xf32, 1>, %n_: i64, %k_: i64, %rs_: i64, %cs_: i64, %info0: memref<1xi32, 1>) kernel {
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
    %in = arith.andi %in_r, %tri : i1
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
}
