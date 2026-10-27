// Complex BLAS kernels of zluda_blas_agx (the C routines; agx_libs/build.rs
// adds f64 copies, `<name>_f64`, for the Z ones), for every AGX generation
// like blas.mlir. Complex numbers are interleaved (re, im) pairs; indices and
// leading dimensions count complex elements. Vectors and column-major
// matrices follow BLAS conventions (blas.mlir). A matrix operation `op` is
// 0 (as stored), 1 (transposed) or 2 (conjugate transposed). Complex scalars
// are passed as (re, im) pairs of arguments.

gpu.module @agx_blas {
  // ---- helpers

  func.func private @agx_cblas_pos(%i: index, %n: index, %inc_: i64) -> index {
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

  // Complex element p of a buffer.
  func.func private @agx_c_ld(%m0: memref<1xf32, 1>, %p: index) -> (f32, f32) {
    %big = arith.constant 4611686018427387904 : index
    %i1 = arith.constant 1 : index
    %m = memref.reinterpret_cast %m0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %r = arith.addi %p, %p : index
    %i = arith.addi %r, %i1 : index
    %vr = memref.load %m[%r] : memref<?xf32, strided<[1]>, 1>
    %vi = memref.load %m[%i] : memref<?xf32, strided<[1]>, 1>
    return %vr, %vi : f32, f32
  }

  func.func private @agx_c_st(%m0: memref<1xf32, 1>, %p: index, %vr: f32, %vi: f32) {
    %big = arith.constant 4611686018427387904 : index
    %i1 = arith.constant 1 : index
    %m = memref.reinterpret_cast %m0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %r = arith.addi %p, %p : index
    %i = arith.addi %r, %i1 : index
    memref.store %vr, %m[%r] : memref<?xf32, strided<[1]>, 1>
    memref.store %vi, %m[%i] : memref<?xf32, strided<[1]>, 1>
    return
  }

  // c + a b
  func.func private @agx_c_fma(%ar: f32, %ai: f32, %br: f32, %bi: f32, %cr: f32, %ci: f32) -> (f32, f32) {
    %t = math.fma %ar, %br, %cr : f32
    %nai = arith.negf %ai : f32
    %rr = math.fma %nai, %bi, %t : f32
    %u = math.fma %ar, %bi, %ci : f32
    %ri = math.fma %ai, %br, %u : f32
    return %rr, %ri : f32, f32
  }

  // Element (r, c) of op(A): A[r + c ld] as stored, A[c + r ld] transposed,
  // and conjugated for op 2.
  func.func private @agx_c_get(%a0: memref<1xf32, 1>, %r: index, %c: index, %ld: index, %op: i64) -> (f32, f32) {
    %z = arith.constant 0 : i64
    %two = arith.constant 2 : i64
    %t = arith.cmpi ne, %op, %z : i64
    %cj = arith.cmpi eq, %op, %two : i64
    %row = arith.select %t, %c, %r : index
    %col = arith.select %t, %r, %c : index
    %cl = arith.muli %col, %ld : index
    %p = arith.addi %cl, %row : index
    %v:2 = func.call @agx_c_ld(%a0, %p) : (memref<1xf32, 1>, index) -> (f32, f32)
    %ni = arith.negf %v#1 : f32
    %im = arith.select %cj, %ni, %v#1 : f32
    return %v#0, %im : f32, f32
  }

  // Element (r, c) of the Hermitian (or, without herm, symmetric) matrix held
  // in the upper (or lower) triangle of A; a Hermitian diagonal is real.
  func.func private @agx_c_herm(%a0: memref<1xf32, 1>, %r: index, %c: index, %ld: index,
                                %upper: i1, %herm: i1) -> (f32, f32) {
    %f0 = arith.constant 0.0 : f32
    %le = arith.cmpi ule, %r, %c : index
    %ge = arith.cmpi uge, %r, %c : index
    %inside = arith.select %upper, %le, %ge : i1
    %row = arith.select %inside, %r, %c : index
    %col = arith.select %inside, %c, %r : index
    %cl = arith.muli %col, %ld : index
    %p = arith.addi %cl, %row : index
    %v:2 = func.call @agx_c_ld(%a0, %p) : (memref<1xf32, 1>, index) -> (f32, f32)
    %outside = arith.cmpi ne, %row, %r : index
    %flip = arith.andi %outside, %herm : i1
    %ni = arith.negf %v#1 : f32
    %im0 = arith.select %flip, %ni, %v#1 : f32
    %diag = arith.cmpi eq, %r, %c : index
    %real = arith.andi %diag, %herm : i1
    %im = arith.select %real, %f0, %im0 : f32
    return %v#0, %im : f32, f32
  }

  // C[p] = alpha s + beta C[p] (C not read when beta is 0); with realdiag, a
  // Hermitian diagonal element: the imaginary parts of C[p] and the result are
  // 0.
  func.func private @agx_c_blend(%c0: memref<1xf32, 1>, %p: index, %sr: f32, %si: f32,
                                 %ar: f32, %ai: f32, %br: f32, %bi: f32, %realdiag: i1) {
    %f0 = arith.constant 0.0 : f32
    %as:2 = func.call @agx_c_fma(%ar, %ai, %sr, %si, %f0, %f0) : (f32, f32, f32, f32, f32, f32) -> (f32, f32)
    %zr = arith.cmpf oeq, %br, %f0 : f32
    %zi = arith.cmpf oeq, %bi, %f0 : f32
    %zero = arith.andi %zr, %zi : i1
    %v:2 = scf.if %zero -> (f32, f32) {
      scf.yield %as#0, %as#1 : f32, f32
    } else {
      %old:2 = func.call @agx_c_ld(%c0, %p) : (memref<1xf32, 1>, index) -> (f32, f32)
      %oi = arith.select %realdiag, %f0, %old#1 : f32
      %r:2 = func.call @agx_c_fma(%br, %bi, %old#0, %oi, %as#0, %as#1) : (f32, f32, f32, f32, f32, f32) -> (f32, f32)
      scf.yield %r#0, %r#1 : f32, f32
    }
    %vi = arith.select %realdiag, %f0, %v#1 : f32
    func.call @agx_c_st(%c0, %p, %v#0, %vi) : (memref<1xf32, 1>, index, f32, f32) -> ()
    return
  }

  // ---- level 3

  // C = alpha op(A) op(B) + beta C, op(A) m x k, op(B) k x n. One thread per
  // element of C (2-D grid).
  gpu.func @agx_cgemm(%a0: memref<1xf32, 1>, %b0: memref<1xf32, 1>, %c0: memref<1xf32, 1>,
                      %m: index, %n: index, %k: index, %lda: index, %ldb: index, %ldc: index,
                      %opa: i64, %opb: i64, %ar: f32, %ai: f32, %br: f32, %bi: f32) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %f0 = arith.constant 0.0 : f32
    %false = arith.constant false
    %i = gpu.global_id x
    %j = gpu.global_id y
    %in_i = arith.cmpi ult, %i, %m : index
    %in_j = arith.cmpi ult, %j, %n : index
    %in = arith.andi %in_i, %in_j : i1
    scf.if %in {
      %s:2 = scf.for %l = %i0 to %k step %i1 iter_args(%sr = %f0, %si = %f0) -> (f32, f32) {
        %u:2 = func.call @agx_c_get(%a0, %i, %l, %lda, %opa) : (memref<1xf32, 1>, index, index, index, i64) -> (f32, f32)
        %v:2 = func.call @agx_c_get(%b0, %l, %j, %ldb, %opb) : (memref<1xf32, 1>, index, index, index, i64) -> (f32, f32)
        %r:2 = func.call @agx_c_fma(%u#0, %u#1, %v#0, %v#1, %sr, %si) : (f32, f32, f32, f32, f32, f32) -> (f32, f32)
        scf.yield %r#0, %r#1 : f32, f32
      }
      %jl = arith.muli %j, %ldc : index
      %p = arith.addi %jl, %i : index
      func.call @agx_c_blend(%c0, %p, %s#0, %s#1, %ar, %ai, %br, %bi, %false)
          : (memref<1xf32, 1>, index, f32, f32, f32, f32, f32, f32, i1) -> ()
    }
    gpu.return
  }

  // hemm (herm) / symm: C = alpha A B + beta C (left: A m x m) or alpha B A +
  // beta C (right: A n x n), A Hermitian or symmetric in its upper or lower
  // triangle. One thread per element of C.
  gpu.func @agx_chemm(%a0: memref<1xf32, 1>, %b0: memref<1xf32, 1>, %c0: memref<1xf32, 1>,
                      %m: index, %n: index, %lda: index, %ldb: index, %ldc: index,
                      %right_: i64, %upper_: i64, %herm_: i64,
                      %ar: f32, %ai: f32, %br: f32, %bi: f32) kernel {
    %z = arith.constant 0 : i64
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %f0 = arith.constant 0.0 : f32
    %false = arith.constant false
    %right = arith.cmpi ne, %right_, %z : i64
    %upper = arith.cmpi ne, %upper_, %z : i64
    %herm = arith.cmpi ne, %herm_, %z : i64
    %i = gpu.global_id x
    %j = gpu.global_id y
    %in_i = arith.cmpi ult, %i, %m : index
    %in_j = arith.cmpi ult, %j, %n : index
    %in = arith.andi %in_i, %in_j : i1
    scf.if %in {
      %len = arith.select %right, %n, %m : index
      %s:2 = scf.for %l = %i0 to %len step %i1 iter_args(%sr = %f0, %si = %f0) -> (f32, f32) {
        // left: A(i, l) B(l, j); right: B(i, l) A(l, j)
        %ar_ = arith.select %right, %l, %i : index
        %ac_ = arith.select %right, %j, %l : index
        %br_ = arith.select %right, %i, %l : index
        %bc_ = arith.select %right, %l, %j : index
        %u:2 = func.call @agx_c_herm(%a0, %ar_, %ac_, %lda, %upper, %herm) : (memref<1xf32, 1>, index, index, index, i1, i1) -> (f32, f32)
        %bcl = arith.muli %bc_, %ldb : index
        %bp = arith.addi %bcl, %br_ : index
        %v:2 = func.call @agx_c_ld(%b0, %bp) : (memref<1xf32, 1>, index) -> (f32, f32)
        %r:2 = func.call @agx_c_fma(%u#0, %u#1, %v#0, %v#1, %sr, %si) : (f32, f32, f32, f32, f32, f32) -> (f32, f32)
        scf.yield %r#0, %r#1 : f32, f32
      }
      %jl = arith.muli %j, %ldc : index
      %p = arith.addi %jl, %i : index
      func.call @agx_c_blend(%c0, %p, %s#0, %s#1, %ar, %ai, %br, %bi, %false)
          : (memref<1xf32, 1>, index, f32, f32, f32, f32, f32, f32, i1) -> ()
    }
    gpu.return
  }

  // Rank-k updates of one triangle of the n x n C, with X = op(A) and
  // Y = op(B) n x k:
  //   C(i, j) = alpha sum X(i, l) c(Y(j, l)) [+ alpha2 sum Y(i, l) c(X(j, l))]
  //             + beta C(i, j)
  // where c conjugates when herm (herk, her2k: the diagonal is real) and the
  // bracket is there when two (her2k, syr2k; B is A otherwise).
  gpu.func @agx_crankk(%a0: memref<1xf32, 1>, %b0: memref<1xf32, 1>, %c0: memref<1xf32, 1>,
                       %n: index, %k: index, %lda: index, %ldb: index, %ldc: index,
                       %upper_: i64, %op: i64, %herm_: i64, %two_: i64,
                       %ar: f32, %ai: f32, %a2r: f32, %a2i: f32, %br: f32, %bi: f32) kernel {
    %z = arith.constant 0 : i64
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %f0 = arith.constant 0.0 : f32
    %upper = arith.cmpi ne, %upper_, %z : i64
    %herm = arith.cmpi ne, %herm_, %z : i64
    %two = arith.cmpi ne, %two_, %z : i64
    %i = gpu.global_id x
    %j = gpu.global_id y
    %in_i = arith.cmpi ult, %i, %n : index
    %in_j = arith.cmpi ult, %j, %n : index
    %le = arith.cmpi ule, %i, %j : index
    %ge = arith.cmpi uge, %i, %j : index
    %tri = arith.select %upper, %le, %ge : i1
    %in0 = arith.andi %in_i, %in_j : i1
    %in = arith.andi %in0, %tri : i1
    scf.if %in {
      %s:4 = scf.for %l = %i0 to %k step %i1 iter_args(%sr = %f0, %si = %f0, %tr = %f0, %ti = %f0) -> (f32, f32, f32, f32) {
        %x:2 = func.call @agx_c_get(%a0, %i, %l, %lda, %op) : (memref<1xf32, 1>, index, index, index, i64) -> (f32, f32)
        %y:2 = func.call @agx_c_get(%b0, %j, %l, %ldb, %op) : (memref<1xf32, 1>, index, index, index, i64) -> (f32, f32)
        %nyi = arith.negf %y#1 : f32
        %yi = arith.select %herm, %nyi, %y#1 : f32
        %r:2 = func.call @agx_c_fma(%x#0, %x#1, %y#0, %yi, %sr, %si) : (f32, f32, f32, f32, f32, f32) -> (f32, f32)
        %t:2 = scf.if %two -> (f32, f32) {
          %y2:2 = func.call @agx_c_get(%b0, %i, %l, %ldb, %op) : (memref<1xf32, 1>, index, index, index, i64) -> (f32, f32)
          %x2:2 = func.call @agx_c_get(%a0, %j, %l, %lda, %op) : (memref<1xf32, 1>, index, index, index, i64) -> (f32, f32)
          %nxi = arith.negf %x2#1 : f32
          %xi = arith.select %herm, %nxi, %x2#1 : f32
          %q:2 = func.call @agx_c_fma(%y2#0, %y2#1, %x2#0, %xi, %tr, %ti) : (f32, f32, f32, f32, f32, f32) -> (f32, f32)
          scf.yield %q#0, %q#1 : f32, f32
        } else {
          scf.yield %tr, %ti : f32, f32
        }
        scf.yield %r#0, %r#1, %t#0, %t#1 : f32, f32, f32, f32
      }
      // alpha s + alpha2 t, then the blend with alpha 1
      %u:2 = func.call @agx_c_fma(%ar, %ai, %s#0, %s#1, %f0, %f0) : (f32, f32, f32, f32, f32, f32) -> (f32, f32)
      %w:2 = func.call @agx_c_fma(%a2r, %a2i, %s#2, %s#3, %u#0, %u#1) : (f32, f32, f32, f32, f32, f32) -> (f32, f32)
      %diag = arith.cmpi eq, %i, %j : index
      %real = arith.andi %diag, %herm : i1
      %f1 = arith.constant 1.0 : f32
      %jl = arith.muli %j, %ldc : index
      %p = arith.addi %jl, %i : index
      func.call @agx_c_blend(%c0, %p, %w#0, %w#1, %f1, %f0, %br, %bi, %real)
          : (memref<1xf32, 1>, index, f32, f32, f32, f32, f32, f32, i1) -> ()
    }
    gpu.return
  }

  // ---- level 2

  // y = alpha M x + beta y (y not read when beta is 0), with M = op(A) of the
  // m x n A (hmode 0), or the Hermitian n x n A in its upper (hmode 1) or
  // lower (hmode 2) triangle (hemv). One thread per element of y.
  gpu.func @agx_cgemv(%a0: memref<1xf32, 1>, %x0: memref<1xf32, 1>, %y0: memref<1xf32, 1>,
                      %m: index, %n: index, %lda: index, %incx: i64, %incy: i64,
                      %op: i64, %hmode: i64, %ar: f32, %ai: f32, %br: f32, %bi: f32) kernel {
    %z = arith.constant 0 : i64
    %c1 = arith.constant 1 : i64
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %f0 = arith.constant 0.0 : f32
    %true = arith.constant true
    %false = arith.constant false
    %trans = arith.cmpi ne, %op, %z : i64
    %herm = arith.cmpi ne, %hmode, %z : i64
    %upper = arith.cmpi eq, %hmode, %c1 : i64
    %ylen0 = arith.select %trans, %n, %m : index
    %xlen0 = arith.select %trans, %m, %n : index
    %ylen = arith.select %herm, %n, %ylen0 : index
    %xlen = arith.select %herm, %n, %xlen0 : index
    %i = gpu.global_id x
    %live = arith.cmpi ult, %i, %ylen : index
    scf.if %live {
      %s:2 = scf.for %l = %i0 to %xlen step %i1 iter_args(%sr = %f0, %si = %f0) -> (f32, f32) {
        %u:2 = scf.if %herm -> (f32, f32) {
          %h:2 = func.call @agx_c_herm(%a0, %i, %l, %lda, %upper, %true) : (memref<1xf32, 1>, index, index, index, i1, i1) -> (f32, f32)
          scf.yield %h#0, %h#1 : f32, f32
        } else {
          %g:2 = func.call @agx_c_get(%a0, %i, %l, %lda, %op) : (memref<1xf32, 1>, index, index, index, i64) -> (f32, f32)
          scf.yield %g#0, %g#1 : f32, f32
        }
        %ix = func.call @agx_cblas_pos(%l, %xlen, %incx) : (index, index, i64) -> index
        %v:2 = func.call @agx_c_ld(%x0, %ix) : (memref<1xf32, 1>, index) -> (f32, f32)
        %r:2 = func.call @agx_c_fma(%u#0, %u#1, %v#0, %v#1, %sr, %si) : (f32, f32, f32, f32, f32, f32) -> (f32, f32)
        scf.yield %r#0, %r#1 : f32, f32
      }
      %iy = func.call @agx_cblas_pos(%i, %ylen, %incy) : (index, index, i64) -> index
      func.call @agx_c_blend(%y0, %iy, %s#0, %s#1, %ar, %ai, %br, %bi, %false)
          : (memref<1xf32, 1>, index, f32, f32, f32, f32, f32, f32, i1) -> ()
    }
    gpu.return
  }

  // y = alpha op(A) x + beta y for op(A) transposed (op 1) or conjugate
  // transposed (op 2) of the m x n A: agx_cgemv's general case (hmode 0) with
  // a SIMD-group per element of y, whose lanes read its column of A together,
  // combined by SIMD-group reductions. The grid has 32 n threads.
  gpu.func @agx_cgemv_t(%a0: memref<1xf32, 1>, %x0: memref<1xf32, 1>, %y0: memref<1xf32, 1>,
                        %m: index, %n: index, %lda: index, %incx: i64, %incy: i64,
                        %op: i64, %ar: f32, %ai: f32, %br: f32, %bi: f32) kernel {
    %i0 = arith.constant 0 : index
    %simd = arith.constant 32 : index
    %f0 = arith.constant 0.0 : f32
    %false = arith.constant false
    %g = gpu.global_id x
    %i = arith.divui %g, %simd : index
    %lane = gpu.lane_id
    // the same for the whole SIMD-group, which reduces together
    %live = arith.cmpi ult, %i, %n : index
    scf.if %live {
      %s:2 = scf.for %l = %lane to %m step %simd iter_args(%sr = %f0, %si = %f0) -> (f32, f32) {
        %u:2 = func.call @agx_c_get(%a0, %i, %l, %lda, %op) : (memref<1xf32, 1>, index, index, index, i64) -> (f32, f32)
        %ix = func.call @agx_cblas_pos(%l, %m, %incx) : (index, index, i64) -> index
        %v:2 = func.call @agx_c_ld(%x0, %ix) : (memref<1xf32, 1>, index) -> (f32, f32)
        %r:2 = func.call @agx_c_fma(%u#0, %u#1, %v#0, %v#1, %sr, %si) : (f32, f32, f32, f32, f32, f32) -> (f32, f32)
        scf.yield %r#0, %r#1 : f32, f32
      }
      %tr = gpu.subgroup_reduce add %s#0 : (f32) -> f32
      %ti = gpu.subgroup_reduce add %s#1 : (f32) -> f32
      %first = arith.cmpi eq, %lane, %i0 : index
      scf.if %first {
        %iy = func.call @agx_cblas_pos(%i, %n, %incy) : (index, index, i64) -> index
        func.call @agx_c_blend(%y0, %iy, %tr, %ti, %ar, %ai, %br, %bi, %false)
            : (memref<1xf32, 1>, index, f32, f32, f32, f32, f32, f32, i1) -> ()
      }
    }
    gpu.return
  }

  // A(i, j) += a1 x_i c(y_j) + a2 y_i c(x_j) over the m x n A (tri 0) or its
  // upper (1) or lower (2) triangle, c conjugating when conj: geru / gerc
  // (a2 0), her (y = x, a2 0) and her2 (a2 = conj(a1)); with herm the diagonal
  // stays real. One thread per element.
  gpu.func @agx_cger(%a0: memref<1xf32, 1>, %x0: memref<1xf32, 1>, %y0: memref<1xf32, 1>,
                     %m: index, %n: index, %lda: index, %incx: i64, %incy: i64,
                     %conj_: i64, %tri_: i64, %herm_: i64,
                     %a1r: f32, %a1i: f32, %a2r: f32, %a2i: f32) kernel {
    %z = arith.constant 0 : i64
    %c1 = arith.constant 1 : i64
    %c2 = arith.constant 2 : i64
    %f0 = arith.constant 0.0 : f32
    %conj = arith.cmpi ne, %conj_, %z : i64
    %herm = arith.cmpi ne, %herm_, %z : i64
    %is_up = arith.cmpi eq, %tri_, %c1 : i64
    %is_lo = arith.cmpi eq, %tri_, %c2 : i64
    %i = gpu.global_id x
    %j = gpu.global_id y
    %in_i = arith.cmpi ult, %i, %m : index
    %in_j = arith.cmpi ult, %j, %n : index
    %le = arith.cmpi ule, %i, %j : index
    %ge = arith.cmpi uge, %i, %j : index
    %bad_up = arith.cmpi ugt, %i, %j : index
    %bad_lo = arith.cmpi ult, %i, %j : index
    %out_up = arith.andi %is_up, %bad_up : i1
    %out_lo = arith.andi %is_lo, %bad_lo : i1
    %out = arith.ori %out_up, %out_lo : i1
    %true = arith.constant true
    %ok = arith.xori %out, %true : i1
    %in0 = arith.andi %in_i, %in_j : i1
    %in = arith.andi %in0, %ok : i1
    scf.if %in {
      %ixi = func.call @agx_cblas_pos(%i, %m, %incx) : (index, index, i64) -> index
      %iyj = func.call @agx_cblas_pos(%j, %n, %incy) : (index, index, i64) -> index
      %xi:2 = func.call @agx_c_ld(%x0, %ixi) : (memref<1xf32, 1>, index) -> (f32, f32)
      %yj:2 = func.call @agx_c_ld(%y0, %iyj) : (memref<1xf32, 1>, index) -> (f32, f32)
      %nyj = arith.negf %yj#1 : f32
      %yji = arith.select %conj, %nyj, %yj#1 : f32
      %p:2 = func.call @agx_c_fma(%xi#0, %xi#1, %yj#0, %yji, %f0, %f0) : (f32, f32, f32, f32, f32, f32) -> (f32, f32)
      %t:2 = func.call @agx_c_fma(%a1r, %a1i, %p#0, %p#1, %f0, %f0) : (f32, f32, f32, f32, f32, f32) -> (f32, f32)
      // the second term (her2): y_i c(x_j)
      %iyi = func.call @agx_cblas_pos(%i, %m, %incy) : (index, index, i64) -> index
      %ixj = func.call @agx_cblas_pos(%j, %n, %incx) : (index, index, i64) -> index
      %two0 = arith.cmpf one, %a2r, %f0 : f32
      %two1 = arith.cmpf one, %a2i, %f0 : f32
      %two = arith.ori %two0, %two1 : i1
      %u:2 = scf.if %two -> (f32, f32) {
        %yi:2 = func.call @agx_c_ld(%y0, %iyi) : (memref<1xf32, 1>, index) -> (f32, f32)
        %xj:2 = func.call @agx_c_ld(%x0, %ixj) : (memref<1xf32, 1>, index) -> (f32, f32)
        %nxj = arith.negf %xj#1 : f32
        %xji = arith.select %conj, %nxj, %xj#1 : f32
        %q:2 = func.call @agx_c_fma(%yi#0, %yi#1, %xj#0, %xji, %f0, %f0) : (f32, f32, f32, f32, f32, f32) -> (f32, f32)
        %w:2 = func.call @agx_c_fma(%a2r, %a2i, %q#0, %q#1, %t#0, %t#1) : (f32, f32, f32, f32, f32, f32) -> (f32, f32)
        scf.yield %w#0, %w#1 : f32, f32
      } else {
        scf.yield %t#0, %t#1 : f32, f32
      }
      %jl = arith.muli %j, %lda : index
      %pidx = arith.addi %jl, %i : index
      %old:2 = func.call @agx_c_ld(%a0, %pidx) : (memref<1xf32, 1>, index) -> (f32, f32)
      %nr = arith.addf %old#0, %u#0 : f32
      %ni0 = arith.addf %old#1, %u#1 : f32
      %diag = arith.cmpi eq, %i, %j : index
      %real = arith.andi %diag, %herm : i1
      %ni = arith.select %real, %f0, %ni0 : f32
      func.call @agx_c_st(%a0, %pidx, %nr, %ni) : (memref<1xf32, 1>, index, f32, f32) -> ()
    }
    gpu.return
  }

  // ---- level 1

  // y = alpha x + y (op 0), x = alpha x (op 1, y unused), y = x (op 2), swap
  // x and y (op 3), with a complex alpha. One thread per element.
  gpu.func @agx_cblas_vec(%x0: memref<1xf32, 1>, %y0: memref<1xf32, 1>,
                          %n: index, %incx: i64, %incy: i64, %op: i64,
                          %ar: f32, %ai: f32) kernel {
    %c0 = arith.constant 0 : i64
    %c1 = arith.constant 1 : i64
    %c3 = arith.constant 3 : i64
    %i = gpu.global_id x
    %live = arith.cmpi ult, %i, %n : index
    scf.if %live {
      %ix = func.call @agx_cblas_pos(%i, %n, %incx) : (index, index, i64) -> index
      %vx:2 = func.call @agx_c_ld(%x0, %ix) : (memref<1xf32, 1>, index) -> (f32, f32)
      %is_scal = arith.cmpi eq, %op, %c1 : i64
      scf.if %is_scal {
        %f0 = arith.constant 0.0 : f32
        %s:2 = func.call @agx_c_fma(%ar, %ai, %vx#0, %vx#1, %f0, %f0) : (f32, f32, f32, f32, f32, f32) -> (f32, f32)
        func.call @agx_c_st(%x0, %ix, %s#0, %s#1) : (memref<1xf32, 1>, index, f32, f32) -> ()
      } else {
        %iy = func.call @agx_cblas_pos(%i, %n, %incy) : (index, index, i64) -> index
        %vy:2 = func.call @agx_c_ld(%y0, %iy) : (memref<1xf32, 1>, index) -> (f32, f32)
        %is_axpy = arith.cmpi eq, %op, %c0 : i64
        %ax:2 = func.call @agx_c_fma(%ar, %ai, %vx#0, %vx#1, %vy#0, %vy#1) : (f32, f32, f32, f32, f32, f32) -> (f32, f32)
        %or = arith.select %is_axpy, %ax#0, %vx#0 : f32
        %oi = arith.select %is_axpy, %ax#1, %vx#1 : f32
        func.call @agx_c_st(%y0, %iy, %or, %oi) : (memref<1xf32, 1>, index, f32, f32) -> ()
        %is_swap = arith.cmpi eq, %op, %c3 : i64
        scf.if %is_swap {
          func.call @agx_c_st(%x0, %ix, %vy#0, %vy#1) : (memref<1xf32, 1>, index, f32, f32) -> ()
        }
      }
    }
    gpu.return
  }

  // Adds |v| to (scale, ssq) with sum x^2 = scale^2 ssq.
  func.func private @agx_cblas_ssq(%scale: f32, %ssq: f32, %v: f32) -> (f32, f32) {
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

  // Partial reductions: thread t < threads folds the elements t, t + threads,
  // ... into part[2 t] and part[2 t + 1]:
  //   op 0, dotu: sum x y; op 1, dotc: sum c(x) y (re, im)
  //   op 2, asum: (sum |re| + |im|, 0)
  //   op 3, nrm2: (scale, ssq) over the real and imaginary parts
  //   op 4 amax / op 5 amin: (|re| + |im| of the first extreme, its index as
  //   i32 bits)
  gpu.func @agx_cblas_reduce(%x0: memref<1xf32, 1>, %y0: memref<1xf32, 1>,
                             %part0: memref<1xf32, 1>, %n: index, %incx: i64,
                             %incy: i64, %op: i64, %threads: index) kernel {
    %c1 = arith.constant 1 : i64
    %c2 = arith.constant 2 : i64
    %c3 = arith.constant 3 : i64
    %c4 = arith.constant 4 : i64
    %f0 = arith.constant 0.0 : f32
    %f1 = arith.constant 1.0 : f32
    %t = gpu.global_id x
    %live = arith.cmpi ult, %t, %threads : index
    scf.if %live {
      %is_dot = arith.cmpi ult, %op, %c2 : i64
      %is_dotc = arith.cmpi eq, %op, %c1 : i64
      %is_nrm = arith.cmpi eq, %op, %c3 : i64
      %is_ext = arith.cmpi uge, %op, %c4 : i64
      %is_max = arith.cmpi eq, %op, %c4 : i64
      %none = arith.constant -1.0 : f32
      %a0 = arith.select %is_ext, %none, %f0 : f32
      %b0 = arith.select %is_nrm, %f1, %f0 : f32
      %res:2 = scf.for %k = %t to %n step %threads iter_args(%a = %a0, %b = %b0) -> (f32, f32) {
        %ix = func.call @agx_cblas_pos(%k, %n, %incx) : (index, index, i64) -> index
        %vx:2 = func.call @agx_c_ld(%x0, %ix) : (memref<1xf32, 1>, index) -> (f32, f32)
        %axr = math.absf %vx#0 : f32
        %axi = math.absf %vx#1 : f32
        %ax = arith.addf %axr, %axi : f32
        %r:2 = scf.if %is_dot -> (f32, f32) {
          %iy = func.call @agx_cblas_pos(%k, %n, %incy) : (index, index, i64) -> index
          %vy:2 = func.call @agx_c_ld(%y0, %iy) : (memref<1xf32, 1>, index) -> (f32, f32)
          %nxi = arith.negf %vx#1 : f32
          %xi = arith.select %is_dotc, %nxi, %vx#1 : f32
          %s:2 = func.call @agx_c_fma(%vx#0, %xi, %vy#0, %vy#1, %a, %b) : (f32, f32, f32, f32, f32, f32) -> (f32, f32)
          scf.yield %s#0, %s#1 : f32, f32
        } else {
          %r2:2 = scf.if %is_nrm -> (f32, f32) {
            %q1:2 = func.call @agx_cblas_ssq(%a, %b, %axr) : (f32, f32, f32) -> (f32, f32)
            %q2:2 = func.call @agx_cblas_ssq(%q1#0, %q1#1, %axi) : (f32, f32, f32) -> (f32, f32)
            scf.yield %q2#0, %q2#1 : f32, f32
          } else {
            %r4:2 = scf.if %is_ext -> (f32, f32) {
              // strictly better, so the first (smallest) index wins
              %gt = arith.cmpf ogt, %ax, %a : f32
              %lt = arith.cmpf olt, %ax, %a : f32
              %first = arith.cmpf olt, %a, %f0 : f32
              %better0 = arith.select %is_max, %gt, %lt : i1
              %better = arith.ori %better0, %first : i1
              %k32 = arith.index_cast %k : index to i32
              %kf = arith.bitcast %k32 : i32 to f32
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
      func.call @agx_c_st(%part0, %t, %res#0, %res#1) : (memref<1xf32, 1>, index, f32, f32) -> ()
    }
    gpu.return
  }

  // Combines the partials of agx_cblas_reduce in order into result[0..1]: the
  // complex dot product, (asum or nrm2, 0) as floats, or for amax and amin the
  // 1-based i32 index (0 when n is 0) in result[0]. One thread.
  gpu.func @agx_cblas_reduce_final(%part0: memref<1xf32, 1>, %res0: memref<1xf32, 1>,
                                   %threads: index, %op: i64) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %c2 = arith.constant 2 : i64
    %c3 = arith.constant 3 : i64
    %c4 = arith.constant 4 : i64
    %f0 = arith.constant 0.0 : f32
    %f1 = arith.constant 1.0 : f32
    %t = gpu.global_id x
    %first_thread = arith.cmpi eq, %t, %i0 : index
    scf.if %first_thread {
      %is_dot = arith.cmpi ult, %op, %c2 : i64
      %is_nrm = arith.cmpi eq, %op, %c3 : i64
      %is_ext = arith.cmpi uge, %op, %c4 : i64
      %is_max = arith.cmpi eq, %op, %c4 : i64
      %none = arith.constant -1.0 : f32
      %a0 = arith.select %is_ext, %none, %f0 : f32
      %b0 = arith.select %is_nrm, %f1, %f0 : f32
      %r:2 = scf.for %k = %i0 to %threads step %i1 iter_args(%a = %a0, %b = %b0) -> (f32, f32) {
        %pv:2 = func.call @agx_c_ld(%part0, %k) : (memref<1xf32, 1>, index) -> (f32, f32)
        %o:2 = scf.if %is_dot -> (f32, f32) {
          %sr = arith.addf %a, %pv#0 : f32
          %si = arith.addf %b, %pv#1 : f32
          scf.yield %sr, %si : f32, f32
        } else {
          %o1:2 = scf.if %is_nrm -> (f32, f32) {
            // (scale, ssq) + (pa, pb): fold pa^2 pb
            %zero = arith.cmpf oeq, %pv#0, %f0 : f32
            %o2:2 = scf.if %zero -> (f32, f32) {
              scf.yield %a, %b : f32, f32
            } else {
              %bigger = arith.cmpf ogt, %pv#0, %a : f32
              %o3:2 = scf.if %bigger -> (f32, f32) {
                %q = arith.divf %a, %pv#0 : f32
                %q2 = arith.mulf %q, %q : f32
                %s = math.fma %b, %q2, %pv#1 : f32
                scf.yield %pv#0, %s : f32, f32
              } else {
                %q = arith.divf %pv#0, %a : f32
                %q2 = arith.mulf %q, %q : f32
                %s = math.fma %pv#1, %q2, %b : f32
                scf.yield %a, %s : f32, f32
              }
              scf.yield %o3#0, %o3#1 : f32, f32
            }
            scf.yield %o2#0, %o2#1 : f32, f32
          } else {
            %o4:2 = scf.if %is_ext -> (f32, f32) {
              %empty = arith.cmpf olt, %pv#0, %f0 : f32
              %first = arith.cmpf olt, %a, %f0 : f32
              %gt = arith.cmpf ogt, %pv#0, %a : f32
              %lt = arith.cmpf olt, %pv#0, %a : f32
              %eq = arith.cmpf oeq, %pv#0, %a : f32
              %ia = arith.bitcast %b : f32 to i32
              %ib = arith.bitcast %pv#1 : f32 to i32
              %earlier = arith.cmpi slt, %ib, %ia : i32
              %tie = arith.andi %eq, %earlier : i1
              %strict = arith.select %is_max, %gt, %lt : i1
              %better0 = arith.ori %strict, %tie : i1
              %better1 = arith.ori %better0, %first : i1
              %true = arith.constant true
              %have = arith.xori %empty, %true : i1
              %better = arith.andi %better1, %have : i1
              %na = arith.select %better, %pv#0, %a : f32
              %nb = arith.select %better, %pv#1, %b : f32
              scf.yield %na, %nb : f32, f32
            } else {
              %s = arith.addf %a, %pv#0 : f32
              scf.yield %s, %b : f32, f32
            }
            scf.yield %o4#0, %o4#1 : f32, f32
          }
          scf.yield %o1#0, %o1#1 : f32, f32
        }
        scf.yield %o#0, %o#1 : f32, f32
      }
      %out:2 = scf.if %is_ext -> (f32, f32) {
        %found = arith.cmpf oge, %r#0, %f0 : f32
        %idx = arith.bitcast %r#1 : f32 to i32
        %c1i = arith.constant 1 : i32
        %c0i = arith.constant 0 : i32
        %one_based = arith.addi %idx, %c1i : i32
        %v = arith.select %found, %one_based, %c0i : i32
        %vf = arith.bitcast %v : i32 to f32
        scf.yield %vf, %f0 : f32, f32
      } else {
        %o5:2 = scf.if %is_nrm -> (f32, f32) {
          %root = math.sqrt %r#1 : f32
          %norm = arith.mulf %r#0, %root : f32
          scf.yield %norm, %f0 : f32, f32
        } else {
          scf.yield %r#0, %r#1 : f32, f32
        }
        scf.yield %o5#0, %o5#1 : f32, f32
      }
      func.call @agx_c_st(%res0, %i0, %out#0, %out#1) : (memref<1xf32, 1>, index, f32, f32) -> ()
    }
    gpu.return
  }

  // ---- triangular and elementwise level 3

  // Element (r, c) of op(A) for the triangular A held in its upper (or lower)
  // triangle: 0 outside it, 1 on a unit diagonal.
  func.func private @agx_c_tri(%a0: memref<1xf32, 1>, %r: index, %c: index, %ld: index,
                               %upper: i1, %unit: i1, %op: i64) -> (f32, f32) {
    %z = arith.constant 0 : i64
    %f0 = arith.constant 0.0 : f32
    %f1 = arith.constant 1.0 : f32
    %t = arith.cmpi ne, %op, %z : i64
    %sr = arith.select %t, %c, %r : index
    %sc = arith.select %t, %r, %c : index
    %le = arith.cmpi ule, %sr, %sc : index
    %ge = arith.cmpi uge, %sr, %sc : index
    %inside = arith.select %upper, %le, %ge : i1
    %diag = arith.cmpi eq, %sr, %sc : index
    %one = arith.andi %diag, %unit : i1
    %v:2 = scf.if %inside -> (f32, f32) {
      %g:2 = func.call @agx_c_get(%a0, %r, %c, %ld, %op) : (memref<1xf32, 1>, index, index, index, i64) -> (f32, f32)
      scf.yield %g#0, %g#1 : f32, f32
    } else {
      scf.yield %f0, %f0 : f32, f32
    }
    %re = arith.select %one, %f1, %v#0 : f32
    %im = arith.select %one, %f0, %v#1 : f32
    return %re, %im : f32, f32
  }

  // a / b
  func.func private @agx_c_div(%ar: f32, %ai: f32, %br: f32, %bi: f32) -> (f32, f32) {
    // Smith's algorithm
    %abr = math.absf %br : f32
    %abi = math.absf %bi : f32
    %big = arith.cmpf oge, %abr, %abi : f32
    %r:2 = scf.if %big -> (f32, f32) {
      %q = arith.divf %bi, %br : f32
      %d = math.fma %bi, %q, %br : f32
      %nr0 = math.fma %ai, %q, %ar : f32
      %naq = arith.mulf %ar, %q : f32
      %ni0 = arith.subf %ai, %naq : f32
      %nr = arith.divf %nr0, %d : f32
      %ni = arith.divf %ni0, %d : f32
      scf.yield %nr, %ni : f32, f32
    } else {
      %q = arith.divf %br, %bi : f32
      %d = math.fma %br, %q, %bi : f32
      %nr0 = math.fma %ar, %q, %ai : f32
      %aiq = arith.mulf %ai, %q : f32
      %ni0 = arith.subf %aiq, %ar : f32
      %nr = arith.divf %nr0, %d : f32
      %ni = arith.divf %ni0, %d : f32
      scf.yield %nr, %ni : f32, f32
    }
    return %r#0, %r#1 : f32, f32
  }

  // trmm: C = alpha op(A) B (left, A m x m) or alpha B op(A) (right, A n x n)
  // for the triangular A. One thread per element of C; C must not be B.
  gpu.func @agx_ctrmm(%a0: memref<1xf32, 1>, %b0: memref<1xf32, 1>, %c0: memref<1xf32, 1>,
                      %m: index, %n: index, %lda: index, %ldb: index, %ldc: index,
                      %right_: i64, %upper_: i64, %op: i64, %unit_: i64,
                      %ar: f32, %ai: f32) kernel {
    %z = arith.constant 0 : i64
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %f0 = arith.constant 0.0 : f32
    %false = arith.constant false
    %right = arith.cmpi ne, %right_, %z : i64
    %upper = arith.cmpi ne, %upper_, %z : i64
    %unit = arith.cmpi ne, %unit_, %z : i64
    %i = gpu.global_id x
    %j = gpu.global_id y
    %in_i = arith.cmpi ult, %i, %m : index
    %in_j = arith.cmpi ult, %j, %n : index
    %in = arith.andi %in_i, %in_j : i1
    scf.if %in {
      %len = arith.select %right, %n, %m : index
      %s:2 = scf.for %l = %i0 to %len step %i1 iter_args(%sr = %f0, %si = %f0) -> (f32, f32) {
        %ar_ = arith.select %right, %l, %i : index
        %ac_ = arith.select %right, %j, %l : index
        %br_ = arith.select %right, %i, %l : index
        %bc_ = arith.select %right, %l, %j : index
        %u:2 = func.call @agx_c_tri(%a0, %ar_, %ac_, %lda, %upper, %unit, %op) : (memref<1xf32, 1>, index, index, index, i1, i1, i64) -> (f32, f32)
        %bcl = arith.muli %bc_, %ldb : index
        %bp = arith.addi %bcl, %br_ : index
        %v:2 = func.call @agx_c_ld(%b0, %bp) : (memref<1xf32, 1>, index) -> (f32, f32)
        %r:2 = func.call @agx_c_fma(%u#0, %u#1, %v#0, %v#1, %sr, %si) : (f32, f32, f32, f32, f32, f32) -> (f32, f32)
        scf.yield %r#0, %r#1 : f32, f32
      }
      %jl = arith.muli %j, %ldc : index
      %p = arith.addi %jl, %i : index
      func.call @agx_c_blend(%c0, %p, %s#0, %s#1, %ar, %ai, %f0, %f0, %false)
          : (memref<1xf32, 1>, index, f32, f32, f32, f32, f32, f32, i1) -> ()
    }
    gpu.return
  }

  // trsm: solves op(A) X = alpha B (left) or X op(A) = alpha B (right) in
  // place in B for the triangular A. One thread per column (left) or row
  // (right) of B, each a triangular system M v = alpha b with M = op(A)
  // (left) or op(A)^T (right), lower when `lower`.
  gpu.func @agx_ctrsm(%a0: memref<1xf32, 1>, %b0: memref<1xf32, 1>,
                      %m: index, %n: index, %lda: index, %ldb: index,
                      %right_: i64, %upper_: i64, %op: i64, %unit_: i64, %lower_: i64,
                      %ar: f32, %ai: f32) kernel {
    %z = arith.constant 0 : i64
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %f0 = arith.constant 0.0 : f32
    %right = arith.cmpi ne, %right_, %z : i64
    %upper = arith.cmpi ne, %upper_, %z : i64
    %unit = arith.cmpi ne, %unit_, %z : i64
    %lower = arith.cmpi ne, %lower_, %z : i64
    %t = gpu.global_id x
    %count = arith.select %right, %m, %n : index
    %len = arith.select %right, %n, %m : index
    %live = arith.cmpi ult, %t, %count : index
    scf.if %live {
      %vs = arith.select %right, %ldb, %i1 : index
      %bs0 = arith.muli %t, %ldb : index
      %base = arith.select %right, %t, %bs0 : index
      scf.for %e = %i0 to %len step %i1 {
        %eo = arith.muli %e, %vs : index
        %p = arith.addi %base, %eo : index
        %v:2 = func.call @agx_c_ld(%b0, %p) : (memref<1xf32, 1>, index) -> (f32, f32)
        %s:2 = func.call @agx_c_fma(%ar, %ai, %v#0, %v#1, %f0, %f0) : (f32, f32, f32, f32, f32, f32) -> (f32, f32)
        func.call @agx_c_st(%b0, %p, %s#0, %s#1) : (memref<1xf32, 1>, index, f32, f32) -> ()
      }
      scf.for %step = %i0 to %len step %i1 {
        %lm1 = arith.subi %len, %i1 : index
        %back = arith.subi %lm1, %step : index
        %r = arith.select %lower, %step, %back : index
        %ro = arith.muli %r, %vs : index
        %pr = arith.addi %base, %ro : index
        %br:2 = func.call @agx_c_ld(%b0, %pr) : (memref<1xf32, 1>, index) -> (f32, f32)
        %d:2 = func.call @agx_ctrsm_m(%a0, %r, %r, %lda, %right, %upper, %op, %unit) : (memref<1xf32, 1>, index, index, index, i1, i1, i64, i1) -> (f32, f32)
        %x:2 = func.call @agx_c_div(%br#0, %br#1, %d#0, %d#1) : (f32, f32, f32, f32) -> (f32, f32)
        func.call @agx_c_st(%b0, %pr, %x#0, %x#1) : (memref<1xf32, 1>, index, f32, f32) -> ()
        %nxr = arith.negf %x#0 : f32
        %nxi = arith.negf %x#1 : f32
        %r1 = arith.addi %r, %i1 : index
        %lo = arith.select %lower, %r1, %i0 : index
        %hi = arith.select %lower, %len, %r : index
        scf.for %s2 = %lo to %hi step %i1 {
          %mv:2 = func.call @agx_ctrsm_m(%a0, %s2, %r, %lda, %right, %upper, %op, %unit) : (memref<1xf32, 1>, index, index, index, i1, i1, i64, i1) -> (f32, f32)
          %so = arith.muli %s2, %vs : index
          %ps = arith.addi %base, %so : index
          %old:2 = func.call @agx_c_ld(%b0, %ps) : (memref<1xf32, 1>, index) -> (f32, f32)
          %nv:2 = func.call @agx_c_fma(%mv#0, %mv#1, %nxr, %nxi, %old#0, %old#1) : (f32, f32, f32, f32, f32, f32) -> (f32, f32)
          func.call @agx_c_st(%b0, %ps, %nv#0, %nv#1) : (memref<1xf32, 1>, index, f32, f32) -> ()
        }
      }
    }
    gpu.return
  }

  // M(r, c): op(A)(r, c) left, op(A)(c, r) right.
  func.func private @agx_ctrsm_m(%a0: memref<1xf32, 1>, %r: index, %c: index, %lda: index,
                                 %right: i1, %upper: i1, %op: i64, %unit: i1) -> (f32, f32) {
    %rr = arith.select %right, %c, %r : index
    %cc = arith.select %right, %r, %c : index
    %v:2 = func.call @agx_c_tri(%a0, %rr, %cc, %lda, %upper, %unit, %op) : (memref<1xf32, 1>, index, index, index, i1, i1, i64) -> (f32, f32)
    return %v#0, %v#1 : f32, f32
  }

  // geam: C = alpha op(A) + beta op(B) (m x n). One thread per element.
  gpu.func @agx_cgeam(%a0: memref<1xf32, 1>, %b0: memref<1xf32, 1>, %c0: memref<1xf32, 1>,
                      %m: index, %n: index, %lda: index, %ldb: index, %ldc: index,
                      %opa: i64, %opb: i64, %ar: f32, %ai: f32, %br: f32, %bi: f32) kernel {
    %f0 = arith.constant 0.0 : f32
    %i = gpu.global_id x
    %j = gpu.global_id y
    %in_i = arith.cmpi ult, %i, %m : index
    %in_j = arith.cmpi ult, %j, %n : index
    %in = arith.andi %in_i, %in_j : i1
    scf.if %in {
      %u:2 = func.call @agx_c_get(%a0, %i, %j, %lda, %opa) : (memref<1xf32, 1>, index, index, index, i64) -> (f32, f32)
      %v:2 = func.call @agx_c_get(%b0, %i, %j, %ldb, %opb) : (memref<1xf32, 1>, index, index, index, i64) -> (f32, f32)
      %s:2 = func.call @agx_c_fma(%ar, %ai, %u#0, %u#1, %f0, %f0) : (f32, f32, f32, f32, f32, f32) -> (f32, f32)
      %w:2 = func.call @agx_c_fma(%br, %bi, %v#0, %v#1, %s#0, %s#1) : (f32, f32, f32, f32, f32, f32) -> (f32, f32)
      %jl = arith.muli %j, %ldc : index
      %p = arith.addi %jl, %i : index
      func.call @agx_c_st(%c0, %p, %w#0, %w#1) : (memref<1xf32, 1>, index, f32, f32) -> ()
    }
    gpu.return
  }

  // dgmm: C = A diag(x) (right) or diag(x) A. One thread per element.
  gpu.func @agx_cdgmm(%a0: memref<1xf32, 1>, %x0: memref<1xf32, 1>, %c0: memref<1xf32, 1>,
                      %m: index, %n: index, %lda: index, %incx: i64, %ldc: index,
                      %right_: i64) kernel {
    %z = arith.constant 0 : i64
    %f0 = arith.constant 0.0 : f32
    %right = arith.cmpi ne, %right_, %z : i64
    %i = gpu.global_id x
    %j = gpu.global_id y
    %in_i = arith.cmpi ult, %i, %m : index
    %in_j = arith.cmpi ult, %j, %n : index
    %in = arith.andi %in_i, %in_j : i1
    scf.if %in {
      %jl = arith.muli %j, %lda : index
      %pa = arith.addi %jl, %i : index
      %u:2 = func.call @agx_c_ld(%a0, %pa) : (memref<1xf32, 1>, index) -> (f32, f32)
      %e = arith.select %right, %j, %i : index
      %len = arith.select %right, %n, %m : index
      %ix = func.call @agx_cblas_pos(%e, %len, %incx) : (index, index, i64) -> index
      %v:2 = func.call @agx_c_ld(%x0, %ix) : (memref<1xf32, 1>, index) -> (f32, f32)
      %w:2 = func.call @agx_c_fma(%u#0, %u#1, %v#0, %v#1, %f0, %f0) : (f32, f32, f32, f32, f32, f32) -> (f32, f32)
      %jc = arith.muli %j, %ldc : index
      %pc = arith.addi %jc, %i : index
      func.call @agx_c_st(%c0, %pc, %w#0, %w#1) : (memref<1xf32, 1>, index, f32, f32) -> ()
    }
    gpu.return
  }
}
