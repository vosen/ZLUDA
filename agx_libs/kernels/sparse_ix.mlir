  // Kernels of sparse.mlir written once for both index widths: build.rs puts
  // a copy for i32 and one for i64 indices into the module, with IXT the index
  // type and _IX the name suffix (_i32 or _i64). Indices are from base.

  // m[k] - base and m[k] = v + base of an index buffer.
  func.func private @agx_sp_ix_IX(%m0: memref<1xIXT, 1>, %k: index, %base: index) -> index {
    %big = arith.constant 4611686018427387904 : index
    %m = memref.reinterpret_cast %m0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xIXT, 1> to memref<?xIXT, strided<[1]>, 1>
    %r = memref.load %m[%k] : memref<?xIXT, strided<[1]>, 1>
    %b = arith.index_cast %r : IXT to index
    %v = arith.subi %b, %base : index
    return %v : index
  }
  func.func private @agx_sp_six_IX(%m0: memref<1xIXT, 1>, %k: index, %v: index, %base: index) {
    %big = arith.constant 4611686018427387904 : index
    %m = memref.reinterpret_cast %m0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xIXT, 1> to memref<?xIXT, strided<[1]>, 1>
    %b = arith.addi %v, %base : index
    %r = arith.index_cast %b : index to IXT
    memref.store %r, %m[%k] : memref<?xIXT, strided<[1]>, 1>
    return
  }

  // part[t] = sum of x[k] y[idx[k]] for k = t, t + threads, ... < nnz
  // (cusparseSpVV; agx_sparse_sum adds the partials).
  gpu.func @agx_spvv_IX(%idx0: memref<1xIXT, 1>, %x0: memref<1xf32, 1>, %y0: memref<1xf32, 1>,
                        %part0: memref<1xf32, 1>, %nnz_: i64, %base_: i64, %threads_: i64) kernel {
    %f0 = arith.constant 0.0 : f32
    %nnz = arith.index_cast %nnz_ : i64 to index
    %base = arith.index_cast %base_ : i64 to index
    %threads = arith.index_cast %threads_ : i64 to index
    %t = gpu.global_id x
    %live = arith.cmpi ult, %t, %threads : index
    scf.if %live {
      %s = scf.for %k = %t to %nnz step %threads iter_args(%a = %f0) -> f32 {
        %i = func.call @agx_sp_ix_IX(%idx0, %k, %base) : (memref<1xIXT, 1>, index, index) -> index
        %xv = func.call @agx_sp_ld(%x0, %k) : (memref<1xf32, 1>, index) -> f32
        %yv = func.call @agx_sp_ld(%y0, %i) : (memref<1xf32, 1>, index) -> f32
        %a1 = math.fma %xv, %yv, %a : f32
        scf.yield %a1 : f32
      }
      func.call @agx_sp_st(%part0, %t, %s) : (memref<1xf32, 1>, index, f32) -> ()
    }
    gpu.return
  }

  // For the entries k < nnz of a sparse vector x (indices idx, values xv) and
  // a dense vector y, one thread per entry:
  //   op 0: y[idx[k]] += alpha xv[k] (cusparseAxpby after agx_sparse_scale)
  //   op 1: xv[k] = y[idx[k]] (cusparseGather)
  //   op 2: y[idx[k]] = xv[k] (cusparseScatter)
  //   op 3: (xv[k], y[idx[k]]) = (c xv + s y, c y - s xv) (cusparseRot, c = alpha, s = beta)
  gpu.func @agx_spvec_IX(%idx0: memref<1xIXT, 1>, %x0: memref<1xf32, 1>, %y0: memref<1xf32, 1>,
                         %nnz_: i64, %base_: i64, %op_: i64, %alpha: f32, %beta: f32) kernel {
    %nnz = arith.index_cast %nnz_ : i64 to index
    %base = arith.index_cast %base_ : i64 to index
    %k = gpu.global_id x
    %live = arith.cmpi ult, %k, %nnz : index
    scf.if %live {
      %i = func.call @agx_sp_ix_IX(%idx0, %k, %base) : (memref<1xIXT, 1>, index, index) -> index
      %xv = func.call @agx_sp_ld(%x0, %k) : (memref<1xf32, 1>, index) -> f32
      %yv = func.call @agx_sp_ld(%y0, %i) : (memref<1xf32, 1>, index) -> f32
      %c1 = arith.constant 1 : i64
      %c2 = arith.constant 2 : i64
      %c3 = arith.constant 3 : i64
      %is_g = arith.cmpi eq, %op_, %c1 : i64
      %is_s = arith.cmpi eq, %op_, %c2 : i64
      %is_r = arith.cmpi eq, %op_, %c3 : i64
      scf.if %is_g {
        func.call @agx_sp_st(%x0, %k, %yv) : (memref<1xf32, 1>, index, f32) -> ()
      } else {
        scf.if %is_s {
          func.call @agx_sp_st(%y0, %i, %xv) : (memref<1xf32, 1>, index, f32) -> ()
        } else {
          scf.if %is_r {
            %cx = arith.mulf %alpha, %xv : f32
            %nx = math.fma %beta, %yv, %cx : f32
            %sx = arith.mulf %beta, %xv : f32
            %cy = arith.mulf %alpha, %yv : f32
            %ny = arith.subf %cy, %sx : f32
            func.call @agx_sp_st(%x0, %k, %nx) : (memref<1xf32, 1>, index, f32) -> ()
            func.call @agx_sp_st(%y0, %i, %ny) : (memref<1xf32, 1>, index, f32) -> ()
          } else {
            %ny = math.fma %alpha, %xv, %yv : f32
            func.call @agx_sp_st(%y0, %i, %ny) : (memref<1xf32, 1>, index, f32) -> ()
          }
        }
      }
    }
    gpu.return
  }

  // D[r rs + c cs] = val[k] for the entries of a sparse matrix over a zeroed
  // D (agx_dense_zero): CSR (coo 0, a holds the row offsets, one thread per
  // row) or COO (coo 1, a holds the rows, one thread per entry; n is the
  // number of rows or of entries).
  gpu.func @agx_sp_to_dense_IX(%a0: memref<1xIXT, 1>, %col0: memref<1xIXT, 1>, %val0: memref<1xf32, 1>,
                               %d0: memref<1xf32, 1>, %n_: i64, %base_: i64, %coo_: i64,
                               %rs_: i64, %cs_: i64) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %n = arith.index_cast %n_ : i64 to index
    %base = arith.index_cast %base_ : i64 to index
    %rs = arith.index_cast %rs_ : i64 to index
    %cs = arith.index_cast %cs_ : i64 to index
    %c0 = arith.constant 0 : i64
    %coo = arith.cmpi ne, %coo_, %c0 : i64
    %t = gpu.global_id x
    %live = arith.cmpi ult, %t, %n : index
    scf.if %live {
      %range:2 = scf.if %coo -> (index, index) {
        %t1 = arith.addi %t, %i1 : index
        scf.yield %t, %t1 : index, index
      } else {
        %lo = func.call @agx_sp_ix_IX(%a0, %t, %base) : (memref<1xIXT, 1>, index, index) -> index
        %t1 = arith.addi %t, %i1 : index
        %hi = func.call @agx_sp_ix_IX(%a0, %t1, %base) : (memref<1xIXT, 1>, index, index) -> index
        scf.yield %lo, %hi : index, index
      }
      scf.for %k = %range#0 to %range#1 step %i1 {
        %r = scf.if %coo -> index {
          %rr = func.call @agx_sp_ix_IX(%a0, %k, %base) : (memref<1xIXT, 1>, index, index) -> index
          scf.yield %rr : index
        } else {
          scf.yield %t : index
        }
        %c = func.call @agx_sp_ix_IX(%col0, %k, %base) : (memref<1xIXT, 1>, index, index) -> index
        %v = func.call @agx_sp_ld(%val0, %k) : (memref<1xf32, 1>, index) -> f32
        %or = arith.muli %r, %rs : index
        %oc = arith.muli %c, %cs : index
        %o = arith.addi %or, %oc : index
        func.call @agx_sp_st(%d0, %o, %v) : (memref<1xf32, 1>, index, f32) -> ()
      }
    }
    gpu.return
  }

  // Dense to CSR (cusparseDenseToSparse), one thread per row i of D (rows x
  // cols, element strides rs and cs):
  //   fill 0: rowptr[i + 1] = the number of nonzeros of row i
  //   fill 1: their columns and values, at rowptr[i] - base on
  gpu.func @agx_dense_to_csr_IX(%d0: memref<1xf32, 1>, %rowptr0: memref<1xIXT, 1>,
                                %col0: memref<1xIXT, 1>, %val0: memref<1xf32, 1>,
                                %rows_: i64, %cols_: i64, %rs_: i64, %cs_: i64,
                                %base_: i64, %fill_: i64) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %f0 = arith.constant 0.0 : f32
    %rows = arith.index_cast %rows_ : i64 to index
    %cols = arith.index_cast %cols_ : i64 to index
    %rs = arith.index_cast %rs_ : i64 to index
    %cs = arith.index_cast %cs_ : i64 to index
    %base = arith.index_cast %base_ : i64 to index
    %c0 = arith.constant 0 : i64
    %true = arith.constant true
    %fill = arith.cmpi ne, %fill_, %c0 : i64
    %i = gpu.global_id x
    %live = arith.cmpi ult, %i, %rows : index
    scf.if %live {
      %start = scf.if %fill -> index {
        %s = func.call @agx_sp_ix_IX(%rowptr0, %i, %base) : (memref<1xIXT, 1>, index, index) -> index
        scf.yield %s : index
      } else {
        scf.yield %i0 : index
      }
      %oi = arith.muli %i, %rs : index
      %end = scf.for %j = %i0 to %cols step %i1 iter_args(%p = %start) -> index {
        %oj = arith.muli %j, %cs : index
        %o = arith.addi %oi, %oj : index
        %v = func.call @agx_sp_ld(%d0, %o) : (memref<1xf32, 1>, index) -> f32
        %nz = arith.cmpf une, %v, %f0 : f32
        %p1 = scf.if %nz -> index {
          scf.if %fill {
            func.call @agx_sp_six_IX(%col0, %p, %j, %base) : (memref<1xIXT, 1>, index, index, index) -> ()
            func.call @agx_sp_st(%val0, %p, %v) : (memref<1xf32, 1>, index, f32) -> ()
          }
          %pn = arith.addi %p, %i1 : index
          scf.yield %pn : index
        } else {
          scf.yield %p : index
        }
        scf.yield %p1 : index
      }
      %not_fill = arith.xori %fill, %true : i1
      scf.if %not_fill {
        %i_1 = arith.addi %i, %i1 : index
        func.call @agx_sp_six_IX(%rowptr0, %i_1, %end, %i0) : (memref<1xIXT, 1>, index, index, index) -> ()
      }
    }
    gpu.return
  }

  // p[0] = base and p[i] = p[i - 1] + p[i] for 0 < i <= n: row offsets from
  // the counts of agx_dense_to_csr. One thread.
  gpu.func @agx_sparse_scan_IX(%p0: memref<1xIXT, 1>, %n_: i64, %base_: i64) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %n = arith.index_cast %n_ : i64 to index
    %base = arith.index_cast %base_ : i64 to index
    %t = gpu.global_id x
    %first = arith.cmpi eq, %t, %i0 : index
    scf.if %first {
      func.call @agx_sp_six_IX(%p0, %i0, %i0, %base) : (memref<1xIXT, 1>, index, index, index) -> ()
      %n1 = arith.addi %n, %i1 : index
      %last = scf.for %i = %i1 to %n1 step %i1 iter_args(%r = %i0) -> index {
        %c = func.call @agx_sp_ix_IX(%p0, %i, %i0) : (memref<1xIXT, 1>, index, index) -> index
        %r1 = arith.addi %r, %c : index
        func.call @agx_sp_six_IX(%p0, %i, %r1, %base) : (memref<1xIXT, 1>, index, index, index) -> ()
        scf.yield %r1 : index
      }
    }
    gpu.return
  }

  // SDDMM: val[k] = alpha sum over t < K of A[i, t] B[t, col[k]] + beta val[k]
  // for the entries k of row i of a CSR matrix, with op(A) (rows x K) and
  // op(B) (K x cols) given by element strides. val is not read when beta is
  // 0. One thread per row.
  gpu.func @agx_sddmm_IX(%rowptr0: memref<1xIXT, 1>, %col0: memref<1xIXT, 1>, %val0: memref<1xf32, 1>,
                         %a0: memref<1xf32, 1>, %b0: memref<1xf32, 1>, %rows_: i64, %k_: i64,
                         %base_: i64, %ars_: i64, %acs_: i64, %brs_: i64, %bcs_: i64,
                         %alpha: f32, %beta: f32) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %f0 = arith.constant 0.0 : f32
    %rows = arith.index_cast %rows_ : i64 to index
    %kk = arith.index_cast %k_ : i64 to index
    %base = arith.index_cast %base_ : i64 to index
    %ars = arith.index_cast %ars_ : i64 to index
    %acs = arith.index_cast %acs_ : i64 to index
    %brs = arith.index_cast %brs_ : i64 to index
    %bcs = arith.index_cast %bcs_ : i64 to index
    %i = gpu.global_id x
    %live = arith.cmpi ult, %i, %rows : index
    scf.if %live {
      %lo = func.call @agx_sp_ix_IX(%rowptr0, %i, %base) : (memref<1xIXT, 1>, index, index) -> index
      %i_1 = arith.addi %i, %i1 : index
      %hi = func.call @agx_sp_ix_IX(%rowptr0, %i_1, %base) : (memref<1xIXT, 1>, index, index) -> index
      %ai = arith.muli %i, %ars : index
      %no_old = arith.cmpf oeq, %beta, %f0 : f32
      scf.for %k = %lo to %hi step %i1 {
        %j = func.call @agx_sp_ix_IX(%col0, %k, %base) : (memref<1xIXT, 1>, index, index) -> index
        %bj = arith.muli %j, %bcs : index
        %dot = scf.for %t = %i0 to %kk step %i1 iter_args(%s = %f0) -> f32 {
          %at = arith.muli %t, %acs : index
          %ao = arith.addi %ai, %at : index
          %bt = arith.muli %t, %brs : index
          %bo = arith.addi %bt, %bj : index
          %av = func.call @agx_sp_ld(%a0, %ao) : (memref<1xf32, 1>, index) -> f32
          %bv = func.call @agx_sp_ld(%b0, %bo) : (memref<1xf32, 1>, index) -> f32
          %s1 = math.fma %av, %bv, %s : f32
          scf.yield %s1 : f32
        }
        %scaled = arith.mulf %dot, %alpha : f32
        %res = scf.if %no_old -> f32 {
          scf.yield %scaled : f32
        } else {
          %old = func.call @agx_sp_ld(%val0, %k) : (memref<1xf32, 1>, index) -> f32
          %ep = math.fma %old, %beta, %scaled : f32
          scf.yield %ep : f32
        }
        func.call @agx_sp_st(%val0, %k, %res) : (memref<1xf32, 1>, index, f32) -> ()
      }
    }
    gpu.return
  }

  // Triangular solve op(A) C = alpha B for a square CSR matrix (cusparseSpSV
  // with n 1 and cusparseSpSM): the lower (upper 0) or upper triangle of A is
  // used, with a unit diagonal when unit is not 0, A^T when trans is not 0,
  // and the other entries are ignored. B and C have element strides (row,
  // column); C may be B. One thread per column of C.
  gpu.func @agx_spsm_IX(%rowptr0: memref<1xIXT, 1>, %col0: memref<1xIXT, 1>, %val0: memref<1xf32, 1>,
                        %b0: memref<1xf32, 1>, %c0: memref<1xf32, 1>, %rows_: i64, %n_: i64,
                        %base_: i64, %brs_: i64, %bcs_: i64, %crs_: i64, %ccs_: i64,
                        %upper_: i64, %unit_: i64, %trans_: i64, %alpha: f32) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %f0 = arith.constant 0.0 : f32
    %f1 = arith.constant 1.0 : f32
    %z = arith.constant 0 : i64
    %rows = arith.index_cast %rows_ : i64 to index
    %n = arith.index_cast %n_ : i64 to index
    %base = arith.index_cast %base_ : i64 to index
    %brs = arith.index_cast %brs_ : i64 to index
    %bcs = arith.index_cast %bcs_ : i64 to index
    %crs = arith.index_cast %crs_ : i64 to index
    %ccs = arith.index_cast %ccs_ : i64 to index
    %upper = arith.cmpi ne, %upper_, %z : i64
    %unit = arith.cmpi ne, %unit_, %z : i64
    %trans = arith.cmpi ne, %trans_, %z : i64
    %true = arith.constant true
    %not_trans = arith.xori %trans, %true : i1
    // backward substitution for an upper triangle of op(A)
    %back = arith.xori %upper, %trans : i1
    %j = gpu.global_id x
    %live = arith.cmpi ult, %j, %n : index
    scf.if %live {
      %bj = arith.muli %j, %bcs : index
      %cj = arith.muli %j, %ccs : index
      // A^T: C = alpha B, then each solved x_i is subtracted from the
      // entries of C its row of A reaches (column oriented)
      scf.if %trans {
        scf.for %i = %i0 to %rows step %i1 {
          %bi = arith.muli %i, %brs : index
          %bo = arith.addi %bi, %bj : index
          %ci = arith.muli %i, %crs : index
          %co = arith.addi %ci, %cj : index
          %bv = func.call @agx_sp_ld(%b0, %bo) : (memref<1xf32, 1>, index) -> f32
          %s = arith.mulf %bv, %alpha : f32
          func.call @agx_sp_st(%c0, %co, %s) : (memref<1xf32, 1>, index, f32) -> ()
        }
      }
      %rows_1 = arith.subi %rows, %i1 : index
      scf.for %r = %i0 to %rows step %i1 {
        %rr = arith.subi %rows_1, %r : index
        %i = arith.select %back, %rr, %r : index
        %lo = func.call @agx_sp_ix_IX(%rowptr0, %i, %base) : (memref<1xIXT, 1>, index, index) -> index
        %i_1 = arith.addi %i, %i1 : index
        %hi = func.call @agx_sp_ix_IX(%rowptr0, %i_1, %base) : (memref<1xIXT, 1>, index, index) -> index
        %ci = arith.muli %i, %crs : index
        %co = arith.addi %ci, %cj : index
        %s0 = scf.if %trans -> f32 {
          scf.yield %f0 : f32
        } else {
          %bi = arith.muli %i, %brs : index
          %bo = arith.addi %bi, %bj : index
          %bv = func.call @agx_sp_ld(%b0, %bo) : (memref<1xf32, 1>, index) -> f32
          %s = arith.mulf %bv, %alpha : f32
          scf.yield %s : f32
        }
        %acc:2 = scf.for %k = %lo to %hi step %i1 iter_args(%s = %s0, %d = %f1) -> (f32, f32) {
          %cc = func.call @agx_sp_ix_IX(%col0, %k, %base) : (memref<1xIXT, 1>, index, index) -> index
          %v = func.call @agx_sp_ld(%val0, %k) : (memref<1xf32, 1>, index) -> f32
          %is_diag = arith.cmpi eq, %cc, %i : index
          %below = arith.cmpi ult, %cc, %i : index
          %above = arith.cmpi ugt, %cc, %i : index
          %in_tri = arith.select %upper, %above, %below : i1
          %use = arith.andi %in_tri, %not_trans : i1
          %s1 = scf.if %use -> f32 {
            %xc = arith.muli %cc, %crs : index
            %xo = arith.addi %xc, %cj : index
            %x = func.call @agx_sp_ld(%c0, %xo) : (memref<1xf32, 1>, index) -> f32
            %vx = arith.mulf %v, %x : f32
            %sn = arith.subf %s, %vx : f32
            scf.yield %sn : f32
          } else {
            scf.yield %s : f32
          }
          %d1 = arith.select %is_diag, %v, %d : f32
          scf.yield %s1, %d1 : f32, f32
        }
        %s = scf.if %trans -> f32 {
          %cv = func.call @agx_sp_ld(%c0, %co) : (memref<1xf32, 1>, index) -> f32
          scf.yield %cv : f32
        } else {
          scf.yield %acc#0 : f32
        }
        %d = arith.select %unit, %f1, %acc#1 : f32
        %x = arith.divf %s, %d : f32
        func.call @agx_sp_st(%c0, %co, %x) : (memref<1xf32, 1>, index, f32) -> ()
        scf.if %trans {
          scf.for %k = %lo to %hi step %i1 {
            %cc = func.call @agx_sp_ix_IX(%col0, %k, %base) : (memref<1xIXT, 1>, index, index) -> index
            %below = arith.cmpi ult, %cc, %i : index
            %above = arith.cmpi ugt, %cc, %i : index
            %in_tri = arith.select %upper, %above, %below : i1
            scf.if %in_tri {
              %v = func.call @agx_sp_ld(%val0, %k) : (memref<1xf32, 1>, index) -> f32
              %yc = arith.muli %cc, %crs : index
              %yo = arith.addi %yc, %cj : index
              %y = func.call @agx_sp_ld(%c0, %yo) : (memref<1xf32, 1>, index) -> f32
              %vx = arith.mulf %v, %x : f32
              %yn = arith.subf %y, %vx : f32
              func.call @agx_sp_st(%c0, %yo, %yn) : (memref<1xf32, 1>, index, f32) -> ()
            }
          }
        }
      }
    }
    gpu.return
  }
