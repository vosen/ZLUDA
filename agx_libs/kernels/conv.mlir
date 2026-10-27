// 2-D convolutions of zluda_dnn_agx (cuDNN), forward and both backward passes,
// for f32 and f16 tensors accumulated in f32: NCHW-shaped tensors with any
// element strides, filters packed as K x C/G x R x S, G groups, padding,
// strides, dilations, and cross-correlation (flip 0) or convolution (flip 1).
// One thread per output element: launch ceil(elements / 256) threadgroups of
// 256 threads. For every AGX generation, like sgemm.mlir.

gpu.module @agx_dnn_conv {
  // y[n, k, p, q] = alpha * sum over c, r, s of x[n, g C/G + c, p sh - ph + r dh, q sw - pw + s dw]
  //   * w[k, c, r, s] + beta * y (the filter flipped in convolution mode); one thread per element of y.
  gpu.func @agx_conv2d_fwd_f32(%x0: memref<1xf32, 1>, %w0: memref<1xf32, 1>, %y0: memref<1xf32, 1>,
      %N_: i32, %C_: i32, %H_: i32, %W_: i32, %K_: i32, %R_: i32, %S_: i32, %P_: i32, %Q_: i32,
      %ph_: i32, %pw_: i32, %sh_: i32, %sw_: i32, %dh_: i32, %dw_: i32, %G_: i32, %flip_: i32,
      %xsn_: i32, %xsc_: i32, %xsh_: i32, %xsw_: i32, %ysn_: i32, %ysc_: i32, %ysh_: i32, %ysw_: i32,
      %alpha: f32, %beta: f32) kernel {
    %N = arith.index_cast %N_ : i32 to index
    %C = arith.index_cast %C_ : i32 to index
    %H = arith.index_cast %H_ : i32 to index
    %W = arith.index_cast %W_ : i32 to index
    %K = arith.index_cast %K_ : i32 to index
    %R = arith.index_cast %R_ : i32 to index
    %S = arith.index_cast %S_ : i32 to index
    %P = arith.index_cast %P_ : i32 to index
    %Q = arith.index_cast %Q_ : i32 to index
    %ph = arith.index_cast %ph_ : i32 to index
    %pw = arith.index_cast %pw_ : i32 to index
    %sh = arith.index_cast %sh_ : i32 to index
    %sw = arith.index_cast %sw_ : i32 to index
    %dh = arith.index_cast %dh_ : i32 to index
    %dw = arith.index_cast %dw_ : i32 to index
    %G = arith.index_cast %G_ : i32 to index
    %flip = arith.index_cast %flip_ : i32 to index
    %xsn = arith.index_cast %xsn_ : i32 to index
    %xsc = arith.index_cast %xsc_ : i32 to index
    %xsh = arith.index_cast %xsh_ : i32 to index
    %xsw = arith.index_cast %xsw_ : i32 to index
    %ysn = arith.index_cast %ysn_ : i32 to index
    %ysc = arith.index_cast %ysc_ : i32 to index
    %ysh = arith.index_cast %ysh_ : i32 to index
    %ysw = arith.index_cast %ysw_ : i32 to index
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %f0 = arith.constant 0.0 : f32
    %big = arith.constant 4611686018427387904 : index
    %Xm = memref.reinterpret_cast %x0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %Wm = memref.reinterpret_cast %w0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %Ym = memref.reinterpret_cast %y0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %cpg = arith.divui %C, %G : index
    %kpg = arith.divui %K, %G : index
    %id = gpu.global_id x
    %no_flip = arith.cmpi eq, %flip, %i0 : index
    %Rm1 = arith.subi %R, %i1 : index
    %Sm1 = arith.subi %S, %i1 : index
    %q = arith.remui %id, %Q : index
    %t1 = arith.divui %id, %Q : index
    %p = arith.remui %t1, %P : index
    %t2 = arith.divui %t1, %P : index
    %k = arith.remui %t2, %K : index
    %n = arith.divui %t2, %K : index
    %live = arith.cmpi ult, %n, %N : index
    scf.if %live {
      %g = arith.divui %k, %kpg : index
      %c_base = arith.muli %g, %cpg : index
      %psh = arith.muli %p, %sh : index
      %h0 = arith.subi %psh, %ph : index
      %qsw = arith.muli %q, %sw : index
      %w_0 = arith.subi %qsw, %pw : index
      %xn = arith.muli %n, %xsn : index
      %acc = scf.for %cc = %i0 to %cpg step %i1 iter_args(%a0 = %f0) -> f32 {
        %c = arith.addi %c_base, %cc : index
        %xc0 = arith.muli %c, %xsc : index
        %xc = arith.addi %xn, %xc0 : index
        %wkc0 = arith.muli %k, %cpg : index
        %wkc = arith.addi %wkc0, %cc : index
        %a1 = scf.for %r = %i0 to %R step %i1 iter_args(%a2 = %a0) -> f32 {
          %rdh = arith.muli %r, %dh : index
          %h = arith.addi %h0, %rdh : index
          %h_ok0 = arith.cmpi sge, %h, %i0 : index
          %h_ok1 = arith.cmpi slt, %h, %H : index
          %h_ok = arith.andi %h_ok0, %h_ok1 : i1
          %a3 = scf.if %h_ok -> f32 {
            %xh0 = arith.muli %h, %xsh : index
            %xh = arith.addi %xc, %xh0 : index
            %rf0 = arith.subi %Rm1, %r : index
            %rf = arith.select %no_flip, %r, %rf0 : index
            %a4 = scf.for %s = %i0 to %S step %i1 iter_args(%a5 = %a2) -> f32 {
              %sdw = arith.muli %s, %dw : index
              %ww = arith.addi %w_0, %sdw : index
              %w_ok0 = arith.cmpi sge, %ww, %i0 : index
              %w_ok1 = arith.cmpi slt, %ww, %W : index
              %w_ok = arith.andi %w_ok0, %w_ok1 : i1
              %a6 = scf.if %w_ok -> f32 {
                %xw0 = arith.muli %ww, %xsw : index
                %xi = arith.addi %xh, %xw0 : index
                %sf0 = arith.subi %Sm1, %s : index
                %sf = arith.select %no_flip, %s, %sf0 : index
                %wi0 = arith.muli %wkc, %R : index
                %wi1 = arith.addi %wi0, %rf : index
                %wi2 = arith.muli %wi1, %S : index
                %wi = arith.addi %wi2, %sf : index
                %xv = memref.load %Xm[%xi] : memref<?xf32, strided<[1]>, 1>
                %wv = memref.load %Wm[%wi] : memref<?xf32, strided<[1]>, 1>
                %a7 = math.fma %xv, %wv, %a5 : f32
                scf.yield %a7 : f32
              } else {
                scf.yield %a5 : f32
              }
              scf.yield %a6 : f32
            }
            scf.yield %a4 : f32
          } else {
            scf.yield %a2 : f32
          }
          scf.yield %a3 : f32
        }
        scf.yield %a1 : f32
      }
      %yn = arith.muli %n, %ysn : index
      %yk = arith.muli %k, %ysc : index
      %yp = arith.muli %p, %ysh : index
      %yq = arith.muli %q, %ysw : index
      %yo0 = arith.addi %yn, %yk : index
      %yo1 = arith.addi %yo0, %yp : index
      %yo = arith.addi %yo1, %yq : index
      %scaled = arith.mulf %acc, %alpha : f32
      %no_old = arith.cmpf oeq, %beta, %f0 : f32
      %res = scf.if %no_old -> f32 {
        scf.yield %scaled : f32
      } else {
        %old = memref.load %Ym[%yo] : memref<?xf32, strided<[1]>, 1>
        %ep_r = math.fma %old, %beta, %scaled : f32
        scf.yield %ep_r : f32
      }
      memref.store %res, %Ym[%yo] : memref<?xf32, strided<[1]>, 1>
    }
    gpu.return
  }
  // dx[n, c, h, w] = alpha * sum over k, r, s with (h + ph - r dh) / sh = p and (w + pw - s dw) / sw = q
  //   integral and in range of dy[n, k, p, q] * w[k, c, r, s] + beta * dx; x is dx and y is dy. One thread per
  //   element of dx.
  gpu.func @agx_conv2d_bwd_data_f32(%x0: memref<1xf32, 1>, %w0: memref<1xf32, 1>, %y0: memref<1xf32, 1>,
      %N_: i32, %C_: i32, %H_: i32, %W_: i32, %K_: i32, %R_: i32, %S_: i32, %P_: i32, %Q_: i32,
      %ph_: i32, %pw_: i32, %sh_: i32, %sw_: i32, %dh_: i32, %dw_: i32, %G_: i32, %flip_: i32,
      %xsn_: i32, %xsc_: i32, %xsh_: i32, %xsw_: i32, %ysn_: i32, %ysc_: i32, %ysh_: i32, %ysw_: i32,
      %alpha: f32, %beta: f32) kernel {
    %N = arith.index_cast %N_ : i32 to index
    %C = arith.index_cast %C_ : i32 to index
    %H = arith.index_cast %H_ : i32 to index
    %W = arith.index_cast %W_ : i32 to index
    %K = arith.index_cast %K_ : i32 to index
    %R = arith.index_cast %R_ : i32 to index
    %S = arith.index_cast %S_ : i32 to index
    %P = arith.index_cast %P_ : i32 to index
    %Q = arith.index_cast %Q_ : i32 to index
    %ph = arith.index_cast %ph_ : i32 to index
    %pw = arith.index_cast %pw_ : i32 to index
    %sh = arith.index_cast %sh_ : i32 to index
    %sw = arith.index_cast %sw_ : i32 to index
    %dh = arith.index_cast %dh_ : i32 to index
    %dw = arith.index_cast %dw_ : i32 to index
    %G = arith.index_cast %G_ : i32 to index
    %flip = arith.index_cast %flip_ : i32 to index
    %xsn = arith.index_cast %xsn_ : i32 to index
    %xsc = arith.index_cast %xsc_ : i32 to index
    %xsh = arith.index_cast %xsh_ : i32 to index
    %xsw = arith.index_cast %xsw_ : i32 to index
    %ysn = arith.index_cast %ysn_ : i32 to index
    %ysc = arith.index_cast %ysc_ : i32 to index
    %ysh = arith.index_cast %ysh_ : i32 to index
    %ysw = arith.index_cast %ysw_ : i32 to index
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %f0 = arith.constant 0.0 : f32
    %big = arith.constant 4611686018427387904 : index
    %Xm = memref.reinterpret_cast %x0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %Wm = memref.reinterpret_cast %w0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %Ym = memref.reinterpret_cast %y0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %cpg = arith.divui %C, %G : index
    %kpg = arith.divui %K, %G : index
    %id = gpu.global_id x
    %no_flip = arith.cmpi eq, %flip, %i0 : index
    %Rm1 = arith.subi %R, %i1 : index
    %Sm1 = arith.subi %S, %i1 : index
    %ww = arith.remui %id, %W : index
    %t1 = arith.divui %id, %W : index
    %h = arith.remui %t1, %H : index
    %t2 = arith.divui %t1, %H : index
    %c = arith.remui %t2, %C : index
    %n = arith.divui %t2, %C : index
    %live = arith.cmpi ult, %n, %N : index
    scf.if %live {
      %g = arith.divui %c, %cpg : index
      %cc = arith.remui %c, %cpg : index
      %k_base = arith.muli %g, %kpg : index
      %hp = arith.addi %h, %ph : index
      %wp = arith.addi %ww, %pw : index
      %yn = arith.muli %n, %ysn : index
      %acc = scf.for %kk = %i0 to %kpg step %i1 iter_args(%a0 = %f0) -> f32 {
        %k = arith.addi %k_base, %kk : index
        %yk0 = arith.muli %k, %ysc : index
        %yk = arith.addi %yn, %yk0 : index
        %wkc0 = arith.muli %k, %cpg : index
        %wkc = arith.addi %wkc0, %cc : index
        %a1 = scf.for %r = %i0 to %R step %i1 iter_args(%a2 = %a0) -> f32 {
          %rdh = arith.muli %r, %dh : index
          %ht = arith.subi %hp, %rdh : index
          %p = arith.divsi %ht, %sh : index
          %pr = arith.remsi %ht, %sh : index
          %h_ok0 = arith.cmpi sge, %ht, %i0 : index
          %h_ok1 = arith.cmpi eq, %pr, %i0 : index
          %h_ok2 = arith.cmpi slt, %p, %P : index
          %h_ok3 = arith.andi %h_ok0, %h_ok1 : i1
          %h_ok = arith.andi %h_ok3, %h_ok2 : i1
          %a3 = scf.if %h_ok -> f32 {
            %yp0 = arith.muli %p, %ysh : index
            %yp = arith.addi %yk, %yp0 : index
            %rf0 = arith.subi %Rm1, %r : index
            %rf = arith.select %no_flip, %r, %rf0 : index
            %a4 = scf.for %s = %i0 to %S step %i1 iter_args(%a5 = %a2) -> f32 {
              %sdw = arith.muli %s, %dw : index
              %wt = arith.subi %wp, %sdw : index
              %q = arith.divsi %wt, %sw : index
              %qr = arith.remsi %wt, %sw : index
              %w_ok0 = arith.cmpi sge, %wt, %i0 : index
              %w_ok1 = arith.cmpi eq, %qr, %i0 : index
              %w_ok2 = arith.cmpi slt, %q, %Q : index
              %w_ok3 = arith.andi %w_ok0, %w_ok1 : i1
              %w_ok = arith.andi %w_ok3, %w_ok2 : i1
              %a6 = scf.if %w_ok -> f32 {
                %yq0 = arith.muli %q, %ysw : index
                %yi = arith.addi %yp, %yq0 : index
                %sf0 = arith.subi %Sm1, %s : index
                %sf = arith.select %no_flip, %s, %sf0 : index
                %wi0 = arith.muli %wkc, %R : index
                %wi1 = arith.addi %wi0, %rf : index
                %wi2 = arith.muli %wi1, %S : index
                %wi = arith.addi %wi2, %sf : index
                %dyv = memref.load %Ym[%yi] : memref<?xf32, strided<[1]>, 1>
                %wv = memref.load %Wm[%wi] : memref<?xf32, strided<[1]>, 1>
                %a7 = math.fma %dyv, %wv, %a5 : f32
                scf.yield %a7 : f32
              } else {
                scf.yield %a5 : f32
              }
              scf.yield %a6 : f32
            }
            scf.yield %a4 : f32
          } else {
            scf.yield %a2 : f32
          }
          scf.yield %a3 : f32
        }
        scf.yield %a1 : f32
      }
      %xn = arith.muli %n, %xsn : index
      %xc = arith.muli %c, %xsc : index
      %xh = arith.muli %h, %xsh : index
      %xw = arith.muli %ww, %xsw : index
      %xo0 = arith.addi %xn, %xc : index
      %xo1 = arith.addi %xo0, %xh : index
      %xo = arith.addi %xo1, %xw : index
      %scaled = arith.mulf %acc, %alpha : f32
      %no_old = arith.cmpf oeq, %beta, %f0 : f32
      %res = scf.if %no_old -> f32 {
        scf.yield %scaled : f32
      } else {
        %old = memref.load %Xm[%xo] : memref<?xf32, strided<[1]>, 1>
        %ep_r = math.fma %old, %beta, %scaled : f32
        scf.yield %ep_r : f32
      }
      memref.store %res, %Xm[%xo] : memref<?xf32, strided<[1]>, 1>
    }
    gpu.return
  }
  // dw[k, c, r, s] = alpha * sum over n, p, q of dy[n, k, p, q] * x[n, g C/G + c, p sh - ph + r dh,
  //   q sw - pw + s dw] + beta * dw, for the stored filter element (r, s) (its tap flipped in convolution mode);
  //   w is dw and y is dy. One thread per element of dw (K x C/G x R x S, packed).
  gpu.func @agx_conv2d_bwd_filter_f32(%x0: memref<1xf32, 1>, %w0: memref<1xf32, 1>, %y0: memref<1xf32, 1>,
      %N_: i32, %C_: i32, %H_: i32, %W_: i32, %K_: i32, %R_: i32, %S_: i32, %P_: i32, %Q_: i32,
      %ph_: i32, %pw_: i32, %sh_: i32, %sw_: i32, %dh_: i32, %dw_: i32, %G_: i32, %flip_: i32,
      %xsn_: i32, %xsc_: i32, %xsh_: i32, %xsw_: i32, %ysn_: i32, %ysc_: i32, %ysh_: i32, %ysw_: i32,
      %alpha: f32, %beta: f32) kernel {
    %N = arith.index_cast %N_ : i32 to index
    %C = arith.index_cast %C_ : i32 to index
    %H = arith.index_cast %H_ : i32 to index
    %W = arith.index_cast %W_ : i32 to index
    %K = arith.index_cast %K_ : i32 to index
    %R = arith.index_cast %R_ : i32 to index
    %S = arith.index_cast %S_ : i32 to index
    %P = arith.index_cast %P_ : i32 to index
    %Q = arith.index_cast %Q_ : i32 to index
    %ph = arith.index_cast %ph_ : i32 to index
    %pw = arith.index_cast %pw_ : i32 to index
    %sh = arith.index_cast %sh_ : i32 to index
    %sw = arith.index_cast %sw_ : i32 to index
    %dh = arith.index_cast %dh_ : i32 to index
    %dw = arith.index_cast %dw_ : i32 to index
    %G = arith.index_cast %G_ : i32 to index
    %flip = arith.index_cast %flip_ : i32 to index
    %xsn = arith.index_cast %xsn_ : i32 to index
    %xsc = arith.index_cast %xsc_ : i32 to index
    %xsh = arith.index_cast %xsh_ : i32 to index
    %xsw = arith.index_cast %xsw_ : i32 to index
    %ysn = arith.index_cast %ysn_ : i32 to index
    %ysc = arith.index_cast %ysc_ : i32 to index
    %ysh = arith.index_cast %ysh_ : i32 to index
    %ysw = arith.index_cast %ysw_ : i32 to index
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %f0 = arith.constant 0.0 : f32
    %big = arith.constant 4611686018427387904 : index
    %Xm = memref.reinterpret_cast %x0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %Wm = memref.reinterpret_cast %w0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %Ym = memref.reinterpret_cast %y0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %cpg = arith.divui %C, %G : index
    %kpg = arith.divui %K, %G : index
    %id = gpu.global_id x
    %no_flip = arith.cmpi eq, %flip, %i0 : index
    %Rm1 = arith.subi %R, %i1 : index
    %Sm1 = arith.subi %S, %i1 : index
    %sf = arith.remui %id, %S : index
    %t1 = arith.divui %id, %S : index
    %rf = arith.remui %t1, %R : index
    %t2 = arith.divui %t1, %R : index
    %cc = arith.remui %t2, %cpg : index
    %k = arith.divui %t2, %cpg : index
    %live = arith.cmpi ult, %k, %K : index
    scf.if %live {
      %g = arith.divui %k, %kpg : index
      %c_base = arith.muli %g, %cpg : index
      %c = arith.addi %c_base, %cc : index
      %r0 = arith.subi %Rm1, %rf : index
      %r = arith.select %no_flip, %rf, %r0 : index
      %s0 = arith.subi %Sm1, %sf : index
      %s = arith.select %no_flip, %sf, %s0 : index
      %rdh = arith.muli %r, %dh : index
      %sdw = arith.muli %s, %dw : index
      %xc = arith.muli %c, %xsc : index
      %yk = arith.muli %k, %ysc : index
      %acc = scf.for %n = %i0 to %N step %i1 iter_args(%a0 = %f0) -> f32 {
        %xn0 = arith.muli %n, %xsn : index
        %xn = arith.addi %xn0, %xc : index
        %yn0 = arith.muli %n, %ysn : index
        %yn = arith.addi %yn0, %yk : index
        %a1 = scf.for %p = %i0 to %P step %i1 iter_args(%a2 = %a0) -> f32 {
          %psh = arith.muli %p, %sh : index
          %h0 = arith.subi %psh, %ph : index
          %h = arith.addi %h0, %rdh : index
          %h_ok0 = arith.cmpi sge, %h, %i0 : index
          %h_ok1 = arith.cmpi slt, %h, %H : index
          %h_ok = arith.andi %h_ok0, %h_ok1 : i1
          %a3 = scf.if %h_ok -> f32 {
            %xh0 = arith.muli %h, %xsh : index
            %xh = arith.addi %xn, %xh0 : index
            %yp0 = arith.muli %p, %ysh : index
            %yp = arith.addi %yn, %yp0 : index
            %a4 = scf.for %q = %i0 to %Q step %i1 iter_args(%a5 = %a2) -> f32 {
              %qsw = arith.muli %q, %sw : index
              %wq0 = arith.subi %qsw, %pw : index
              %ww = arith.addi %wq0, %sdw : index
              %w_ok0 = arith.cmpi sge, %ww, %i0 : index
              %w_ok1 = arith.cmpi slt, %ww, %W : index
              %w_ok = arith.andi %w_ok0, %w_ok1 : i1
              %a6 = scf.if %w_ok -> f32 {
                %xw0 = arith.muli %ww, %xsw : index
                %xi = arith.addi %xh, %xw0 : index
                %yq0 = arith.muli %q, %ysw : index
                %yi = arith.addi %yp, %yq0 : index
                %xv = memref.load %Xm[%xi] : memref<?xf32, strided<[1]>, 1>
                %dyv = memref.load %Ym[%yi] : memref<?xf32, strided<[1]>, 1>
                %a7 = math.fma %xv, %dyv, %a5 : f32
                scf.yield %a7 : f32
              } else {
                scf.yield %a5 : f32
              }
              scf.yield %a6 : f32
            }
            scf.yield %a4 : f32
          } else {
            scf.yield %a2 : f32
          }
          scf.yield %a3 : f32
        }
        scf.yield %a1 : f32
      }
      %wo0 = arith.muli %k, %cpg : index
      %wo1 = arith.addi %wo0, %cc : index
      %wo2 = arith.muli %wo1, %R : index
      %wo3 = arith.addi %wo2, %rf : index
      %wo4 = arith.muli %wo3, %S : index
      %wo = arith.addi %wo4, %sf : index
      %scaled = arith.mulf %acc, %alpha : f32
      %no_old = arith.cmpf oeq, %beta, %f0 : f32
      %res = scf.if %no_old -> f32 {
        scf.yield %scaled : f32
      } else {
        %old = memref.load %Wm[%wo] : memref<?xf32, strided<[1]>, 1>
        %ep_r = math.fma %old, %beta, %scaled : f32
        scf.yield %ep_r : f32
      }
      memref.store %res, %Wm[%wo] : memref<?xf32, strided<[1]>, 1>
    }
    gpu.return
  }
  // y[n, k, p, q] = alpha * sum over c, r, s of x[n, g C/G + c, p sh - ph + r dh, q sw - pw + s dw]
  //   * w[k, c, r, s] + beta * y (the filter flipped in convolution mode); one thread per element of y.
  gpu.func @agx_conv2d_fwd_f16(%x0: memref<1xf16, 1>, %w0: memref<1xf16, 1>, %y0: memref<1xf16, 1>,
      %N_: i32, %C_: i32, %H_: i32, %W_: i32, %K_: i32, %R_: i32, %S_: i32, %P_: i32, %Q_: i32,
      %ph_: i32, %pw_: i32, %sh_: i32, %sw_: i32, %dh_: i32, %dw_: i32, %G_: i32, %flip_: i32,
      %xsn_: i32, %xsc_: i32, %xsh_: i32, %xsw_: i32, %ysn_: i32, %ysc_: i32, %ysh_: i32, %ysw_: i32,
      %alpha: f32, %beta: f32) kernel {
    %N = arith.index_cast %N_ : i32 to index
    %C = arith.index_cast %C_ : i32 to index
    %H = arith.index_cast %H_ : i32 to index
    %W = arith.index_cast %W_ : i32 to index
    %K = arith.index_cast %K_ : i32 to index
    %R = arith.index_cast %R_ : i32 to index
    %S = arith.index_cast %S_ : i32 to index
    %P = arith.index_cast %P_ : i32 to index
    %Q = arith.index_cast %Q_ : i32 to index
    %ph = arith.index_cast %ph_ : i32 to index
    %pw = arith.index_cast %pw_ : i32 to index
    %sh = arith.index_cast %sh_ : i32 to index
    %sw = arith.index_cast %sw_ : i32 to index
    %dh = arith.index_cast %dh_ : i32 to index
    %dw = arith.index_cast %dw_ : i32 to index
    %G = arith.index_cast %G_ : i32 to index
    %flip = arith.index_cast %flip_ : i32 to index
    %xsn = arith.index_cast %xsn_ : i32 to index
    %xsc = arith.index_cast %xsc_ : i32 to index
    %xsh = arith.index_cast %xsh_ : i32 to index
    %xsw = arith.index_cast %xsw_ : i32 to index
    %ysn = arith.index_cast %ysn_ : i32 to index
    %ysc = arith.index_cast %ysc_ : i32 to index
    %ysh = arith.index_cast %ysh_ : i32 to index
    %ysw = arith.index_cast %ysw_ : i32 to index
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %f0 = arith.constant 0.0 : f32
    %big = arith.constant 4611686018427387904 : index
    %Xm = memref.reinterpret_cast %x0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf16, 1> to memref<?xf16, strided<[1]>, 1>
    %Wm = memref.reinterpret_cast %w0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf16, 1> to memref<?xf16, strided<[1]>, 1>
    %Ym = memref.reinterpret_cast %y0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf16, 1> to memref<?xf16, strided<[1]>, 1>
    %cpg = arith.divui %C, %G : index
    %kpg = arith.divui %K, %G : index
    %id = gpu.global_id x
    %no_flip = arith.cmpi eq, %flip, %i0 : index
    %Rm1 = arith.subi %R, %i1 : index
    %Sm1 = arith.subi %S, %i1 : index
    %q = arith.remui %id, %Q : index
    %t1 = arith.divui %id, %Q : index
    %p = arith.remui %t1, %P : index
    %t2 = arith.divui %t1, %P : index
    %k = arith.remui %t2, %K : index
    %n = arith.divui %t2, %K : index
    %live = arith.cmpi ult, %n, %N : index
    scf.if %live {
      %g = arith.divui %k, %kpg : index
      %c_base = arith.muli %g, %cpg : index
      %psh = arith.muli %p, %sh : index
      %h0 = arith.subi %psh, %ph : index
      %qsw = arith.muli %q, %sw : index
      %w_0 = arith.subi %qsw, %pw : index
      %xn = arith.muli %n, %xsn : index
      %acc = scf.for %cc = %i0 to %cpg step %i1 iter_args(%a0 = %f0) -> f32 {
        %c = arith.addi %c_base, %cc : index
        %xc0 = arith.muli %c, %xsc : index
        %xc = arith.addi %xn, %xc0 : index
        %wkc0 = arith.muli %k, %cpg : index
        %wkc = arith.addi %wkc0, %cc : index
        %a1 = scf.for %r = %i0 to %R step %i1 iter_args(%a2 = %a0) -> f32 {
          %rdh = arith.muli %r, %dh : index
          %h = arith.addi %h0, %rdh : index
          %h_ok0 = arith.cmpi sge, %h, %i0 : index
          %h_ok1 = arith.cmpi slt, %h, %H : index
          %h_ok = arith.andi %h_ok0, %h_ok1 : i1
          %a3 = scf.if %h_ok -> f32 {
            %xh0 = arith.muli %h, %xsh : index
            %xh = arith.addi %xc, %xh0 : index
            %rf0 = arith.subi %Rm1, %r : index
            %rf = arith.select %no_flip, %r, %rf0 : index
            %a4 = scf.for %s = %i0 to %S step %i1 iter_args(%a5 = %a2) -> f32 {
              %sdw = arith.muli %s, %dw : index
              %ww = arith.addi %w_0, %sdw : index
              %w_ok0 = arith.cmpi sge, %ww, %i0 : index
              %w_ok1 = arith.cmpi slt, %ww, %W : index
              %w_ok = arith.andi %w_ok0, %w_ok1 : i1
              %a6 = scf.if %w_ok -> f32 {
                %xw0 = arith.muli %ww, %xsw : index
                %xi = arith.addi %xh, %xw0 : index
                %sf0 = arith.subi %Sm1, %s : index
                %sf = arith.select %no_flip, %s, %sf0 : index
                %wi0 = arith.muli %wkc, %R : index
                %wi1 = arith.addi %wi0, %rf : index
                %wi2 = arith.muli %wi1, %S : index
                %wi = arith.addi %wi2, %sf : index
                %xv_h = memref.load %Xm[%xi] : memref<?xf16, strided<[1]>, 1>
                %xv = arith.extf %xv_h : f16 to f32
                %wv_h = memref.load %Wm[%wi] : memref<?xf16, strided<[1]>, 1>
                %wv = arith.extf %wv_h : f16 to f32
                %a7 = math.fma %xv, %wv, %a5 : f32
                scf.yield %a7 : f32
              } else {
                scf.yield %a5 : f32
              }
              scf.yield %a6 : f32
            }
            scf.yield %a4 : f32
          } else {
            scf.yield %a2 : f32
          }
          scf.yield %a3 : f32
        }
        scf.yield %a1 : f32
      }
      %yn = arith.muli %n, %ysn : index
      %yk = arith.muli %k, %ysc : index
      %yp = arith.muli %p, %ysh : index
      %yq = arith.muli %q, %ysw : index
      %yo0 = arith.addi %yn, %yk : index
      %yo1 = arith.addi %yo0, %yp : index
      %yo = arith.addi %yo1, %yq : index
      %scaled = arith.mulf %acc, %alpha : f32
      %no_old = arith.cmpf oeq, %beta, %f0 : f32
      %res = scf.if %no_old -> f32 {
        scf.yield %scaled : f32
      } else {
        %old_h = memref.load %Ym[%yo] : memref<?xf16, strided<[1]>, 1>
        %old = arith.extf %old_h : f16 to f32
        %ep_r = math.fma %old, %beta, %scaled : f32
        scf.yield %ep_r : f32
      }
      %res_h = arith.truncf %res : f32 to f16
      memref.store %res_h, %Ym[%yo] : memref<?xf16, strided<[1]>, 1>
    }
    gpu.return
  }
  // dx[n, c, h, w] = alpha * sum over k, r, s with (h + ph - r dh) / sh = p and (w + pw - s dw) / sw = q
  //   integral and in range of dy[n, k, p, q] * w[k, c, r, s] + beta * dx; x is dx and y is dy. One thread per
  //   element of dx.
  gpu.func @agx_conv2d_bwd_data_f16(%x0: memref<1xf16, 1>, %w0: memref<1xf16, 1>, %y0: memref<1xf16, 1>,
      %N_: i32, %C_: i32, %H_: i32, %W_: i32, %K_: i32, %R_: i32, %S_: i32, %P_: i32, %Q_: i32,
      %ph_: i32, %pw_: i32, %sh_: i32, %sw_: i32, %dh_: i32, %dw_: i32, %G_: i32, %flip_: i32,
      %xsn_: i32, %xsc_: i32, %xsh_: i32, %xsw_: i32, %ysn_: i32, %ysc_: i32, %ysh_: i32, %ysw_: i32,
      %alpha: f32, %beta: f32) kernel {
    %N = arith.index_cast %N_ : i32 to index
    %C = arith.index_cast %C_ : i32 to index
    %H = arith.index_cast %H_ : i32 to index
    %W = arith.index_cast %W_ : i32 to index
    %K = arith.index_cast %K_ : i32 to index
    %R = arith.index_cast %R_ : i32 to index
    %S = arith.index_cast %S_ : i32 to index
    %P = arith.index_cast %P_ : i32 to index
    %Q = arith.index_cast %Q_ : i32 to index
    %ph = arith.index_cast %ph_ : i32 to index
    %pw = arith.index_cast %pw_ : i32 to index
    %sh = arith.index_cast %sh_ : i32 to index
    %sw = arith.index_cast %sw_ : i32 to index
    %dh = arith.index_cast %dh_ : i32 to index
    %dw = arith.index_cast %dw_ : i32 to index
    %G = arith.index_cast %G_ : i32 to index
    %flip = arith.index_cast %flip_ : i32 to index
    %xsn = arith.index_cast %xsn_ : i32 to index
    %xsc = arith.index_cast %xsc_ : i32 to index
    %xsh = arith.index_cast %xsh_ : i32 to index
    %xsw = arith.index_cast %xsw_ : i32 to index
    %ysn = arith.index_cast %ysn_ : i32 to index
    %ysc = arith.index_cast %ysc_ : i32 to index
    %ysh = arith.index_cast %ysh_ : i32 to index
    %ysw = arith.index_cast %ysw_ : i32 to index
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %f0 = arith.constant 0.0 : f32
    %big = arith.constant 4611686018427387904 : index
    %Xm = memref.reinterpret_cast %x0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf16, 1> to memref<?xf16, strided<[1]>, 1>
    %Wm = memref.reinterpret_cast %w0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf16, 1> to memref<?xf16, strided<[1]>, 1>
    %Ym = memref.reinterpret_cast %y0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf16, 1> to memref<?xf16, strided<[1]>, 1>
    %cpg = arith.divui %C, %G : index
    %kpg = arith.divui %K, %G : index
    %id = gpu.global_id x
    %no_flip = arith.cmpi eq, %flip, %i0 : index
    %Rm1 = arith.subi %R, %i1 : index
    %Sm1 = arith.subi %S, %i1 : index
    %ww = arith.remui %id, %W : index
    %t1 = arith.divui %id, %W : index
    %h = arith.remui %t1, %H : index
    %t2 = arith.divui %t1, %H : index
    %c = arith.remui %t2, %C : index
    %n = arith.divui %t2, %C : index
    %live = arith.cmpi ult, %n, %N : index
    scf.if %live {
      %g = arith.divui %c, %cpg : index
      %cc = arith.remui %c, %cpg : index
      %k_base = arith.muli %g, %kpg : index
      %hp = arith.addi %h, %ph : index
      %wp = arith.addi %ww, %pw : index
      %yn = arith.muli %n, %ysn : index
      %acc = scf.for %kk = %i0 to %kpg step %i1 iter_args(%a0 = %f0) -> f32 {
        %k = arith.addi %k_base, %kk : index
        %yk0 = arith.muli %k, %ysc : index
        %yk = arith.addi %yn, %yk0 : index
        %wkc0 = arith.muli %k, %cpg : index
        %wkc = arith.addi %wkc0, %cc : index
        %a1 = scf.for %r = %i0 to %R step %i1 iter_args(%a2 = %a0) -> f32 {
          %rdh = arith.muli %r, %dh : index
          %ht = arith.subi %hp, %rdh : index
          %p = arith.divsi %ht, %sh : index
          %pr = arith.remsi %ht, %sh : index
          %h_ok0 = arith.cmpi sge, %ht, %i0 : index
          %h_ok1 = arith.cmpi eq, %pr, %i0 : index
          %h_ok2 = arith.cmpi slt, %p, %P : index
          %h_ok3 = arith.andi %h_ok0, %h_ok1 : i1
          %h_ok = arith.andi %h_ok3, %h_ok2 : i1
          %a3 = scf.if %h_ok -> f32 {
            %yp0 = arith.muli %p, %ysh : index
            %yp = arith.addi %yk, %yp0 : index
            %rf0 = arith.subi %Rm1, %r : index
            %rf = arith.select %no_flip, %r, %rf0 : index
            %a4 = scf.for %s = %i0 to %S step %i1 iter_args(%a5 = %a2) -> f32 {
              %sdw = arith.muli %s, %dw : index
              %wt = arith.subi %wp, %sdw : index
              %q = arith.divsi %wt, %sw : index
              %qr = arith.remsi %wt, %sw : index
              %w_ok0 = arith.cmpi sge, %wt, %i0 : index
              %w_ok1 = arith.cmpi eq, %qr, %i0 : index
              %w_ok2 = arith.cmpi slt, %q, %Q : index
              %w_ok3 = arith.andi %w_ok0, %w_ok1 : i1
              %w_ok = arith.andi %w_ok3, %w_ok2 : i1
              %a6 = scf.if %w_ok -> f32 {
                %yq0 = arith.muli %q, %ysw : index
                %yi = arith.addi %yp, %yq0 : index
                %sf0 = arith.subi %Sm1, %s : index
                %sf = arith.select %no_flip, %s, %sf0 : index
                %wi0 = arith.muli %wkc, %R : index
                %wi1 = arith.addi %wi0, %rf : index
                %wi2 = arith.muli %wi1, %S : index
                %wi = arith.addi %wi2, %sf : index
                %dyv_h = memref.load %Ym[%yi] : memref<?xf16, strided<[1]>, 1>
                %dyv = arith.extf %dyv_h : f16 to f32
                %wv_h = memref.load %Wm[%wi] : memref<?xf16, strided<[1]>, 1>
                %wv = arith.extf %wv_h : f16 to f32
                %a7 = math.fma %dyv, %wv, %a5 : f32
                scf.yield %a7 : f32
              } else {
                scf.yield %a5 : f32
              }
              scf.yield %a6 : f32
            }
            scf.yield %a4 : f32
          } else {
            scf.yield %a2 : f32
          }
          scf.yield %a3 : f32
        }
        scf.yield %a1 : f32
      }
      %xn = arith.muli %n, %xsn : index
      %xc = arith.muli %c, %xsc : index
      %xh = arith.muli %h, %xsh : index
      %xw = arith.muli %ww, %xsw : index
      %xo0 = arith.addi %xn, %xc : index
      %xo1 = arith.addi %xo0, %xh : index
      %xo = arith.addi %xo1, %xw : index
      %scaled = arith.mulf %acc, %alpha : f32
      %no_old = arith.cmpf oeq, %beta, %f0 : f32
      %res = scf.if %no_old -> f32 {
        scf.yield %scaled : f32
      } else {
        %old_h = memref.load %Xm[%xo] : memref<?xf16, strided<[1]>, 1>
        %old = arith.extf %old_h : f16 to f32
        %ep_r = math.fma %old, %beta, %scaled : f32
        scf.yield %ep_r : f32
      }
      %res_h = arith.truncf %res : f32 to f16
      memref.store %res_h, %Xm[%xo] : memref<?xf16, strided<[1]>, 1>
    }
    gpu.return
  }
  // dw[k, c, r, s] = alpha * sum over n, p, q of dy[n, k, p, q] * x[n, g C/G + c, p sh - ph + r dh,
  //   q sw - pw + s dw] + beta * dw, for the stored filter element (r, s) (its tap flipped in convolution mode);
  //   w is dw and y is dy. One thread per element of dw (K x C/G x R x S, packed).
  gpu.func @agx_conv2d_bwd_filter_f16(%x0: memref<1xf16, 1>, %w0: memref<1xf16, 1>, %y0: memref<1xf16, 1>,
      %N_: i32, %C_: i32, %H_: i32, %W_: i32, %K_: i32, %R_: i32, %S_: i32, %P_: i32, %Q_: i32,
      %ph_: i32, %pw_: i32, %sh_: i32, %sw_: i32, %dh_: i32, %dw_: i32, %G_: i32, %flip_: i32,
      %xsn_: i32, %xsc_: i32, %xsh_: i32, %xsw_: i32, %ysn_: i32, %ysc_: i32, %ysh_: i32, %ysw_: i32,
      %alpha: f32, %beta: f32) kernel {
    %N = arith.index_cast %N_ : i32 to index
    %C = arith.index_cast %C_ : i32 to index
    %H = arith.index_cast %H_ : i32 to index
    %W = arith.index_cast %W_ : i32 to index
    %K = arith.index_cast %K_ : i32 to index
    %R = arith.index_cast %R_ : i32 to index
    %S = arith.index_cast %S_ : i32 to index
    %P = arith.index_cast %P_ : i32 to index
    %Q = arith.index_cast %Q_ : i32 to index
    %ph = arith.index_cast %ph_ : i32 to index
    %pw = arith.index_cast %pw_ : i32 to index
    %sh = arith.index_cast %sh_ : i32 to index
    %sw = arith.index_cast %sw_ : i32 to index
    %dh = arith.index_cast %dh_ : i32 to index
    %dw = arith.index_cast %dw_ : i32 to index
    %G = arith.index_cast %G_ : i32 to index
    %flip = arith.index_cast %flip_ : i32 to index
    %xsn = arith.index_cast %xsn_ : i32 to index
    %xsc = arith.index_cast %xsc_ : i32 to index
    %xsh = arith.index_cast %xsh_ : i32 to index
    %xsw = arith.index_cast %xsw_ : i32 to index
    %ysn = arith.index_cast %ysn_ : i32 to index
    %ysc = arith.index_cast %ysc_ : i32 to index
    %ysh = arith.index_cast %ysh_ : i32 to index
    %ysw = arith.index_cast %ysw_ : i32 to index
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %f0 = arith.constant 0.0 : f32
    %big = arith.constant 4611686018427387904 : index
    %Xm = memref.reinterpret_cast %x0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf16, 1> to memref<?xf16, strided<[1]>, 1>
    %Wm = memref.reinterpret_cast %w0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf16, 1> to memref<?xf16, strided<[1]>, 1>
    %Ym = memref.reinterpret_cast %y0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf16, 1> to memref<?xf16, strided<[1]>, 1>
    %cpg = arith.divui %C, %G : index
    %kpg = arith.divui %K, %G : index
    %id = gpu.global_id x
    %no_flip = arith.cmpi eq, %flip, %i0 : index
    %Rm1 = arith.subi %R, %i1 : index
    %Sm1 = arith.subi %S, %i1 : index
    %sf = arith.remui %id, %S : index
    %t1 = arith.divui %id, %S : index
    %rf = arith.remui %t1, %R : index
    %t2 = arith.divui %t1, %R : index
    %cc = arith.remui %t2, %cpg : index
    %k = arith.divui %t2, %cpg : index
    %live = arith.cmpi ult, %k, %K : index
    scf.if %live {
      %g = arith.divui %k, %kpg : index
      %c_base = arith.muli %g, %cpg : index
      %c = arith.addi %c_base, %cc : index
      %r0 = arith.subi %Rm1, %rf : index
      %r = arith.select %no_flip, %rf, %r0 : index
      %s0 = arith.subi %Sm1, %sf : index
      %s = arith.select %no_flip, %sf, %s0 : index
      %rdh = arith.muli %r, %dh : index
      %sdw = arith.muli %s, %dw : index
      %xc = arith.muli %c, %xsc : index
      %yk = arith.muli %k, %ysc : index
      %acc = scf.for %n = %i0 to %N step %i1 iter_args(%a0 = %f0) -> f32 {
        %xn0 = arith.muli %n, %xsn : index
        %xn = arith.addi %xn0, %xc : index
        %yn0 = arith.muli %n, %ysn : index
        %yn = arith.addi %yn0, %yk : index
        %a1 = scf.for %p = %i0 to %P step %i1 iter_args(%a2 = %a0) -> f32 {
          %psh = arith.muli %p, %sh : index
          %h0 = arith.subi %psh, %ph : index
          %h = arith.addi %h0, %rdh : index
          %h_ok0 = arith.cmpi sge, %h, %i0 : index
          %h_ok1 = arith.cmpi slt, %h, %H : index
          %h_ok = arith.andi %h_ok0, %h_ok1 : i1
          %a3 = scf.if %h_ok -> f32 {
            %xh0 = arith.muli %h, %xsh : index
            %xh = arith.addi %xn, %xh0 : index
            %yp0 = arith.muli %p, %ysh : index
            %yp = arith.addi %yn, %yp0 : index
            %a4 = scf.for %q = %i0 to %Q step %i1 iter_args(%a5 = %a2) -> f32 {
              %qsw = arith.muli %q, %sw : index
              %wq0 = arith.subi %qsw, %pw : index
              %ww = arith.addi %wq0, %sdw : index
              %w_ok0 = arith.cmpi sge, %ww, %i0 : index
              %w_ok1 = arith.cmpi slt, %ww, %W : index
              %w_ok = arith.andi %w_ok0, %w_ok1 : i1
              %a6 = scf.if %w_ok -> f32 {
                %xw0 = arith.muli %ww, %xsw : index
                %xi = arith.addi %xh, %xw0 : index
                %yq0 = arith.muli %q, %ysw : index
                %yi = arith.addi %yp, %yq0 : index
                %xv_h = memref.load %Xm[%xi] : memref<?xf16, strided<[1]>, 1>
                %xv = arith.extf %xv_h : f16 to f32
                %dyv_h = memref.load %Ym[%yi] : memref<?xf16, strided<[1]>, 1>
                %dyv = arith.extf %dyv_h : f16 to f32
                %a7 = math.fma %xv, %dyv, %a5 : f32
                scf.yield %a7 : f32
              } else {
                scf.yield %a5 : f32
              }
              scf.yield %a6 : f32
            }
            scf.yield %a4 : f32
          } else {
            scf.yield %a2 : f32
          }
          scf.yield %a3 : f32
        }
        scf.yield %a1 : f32
      }
      %wo0 = arith.muli %k, %cpg : index
      %wo1 = arith.addi %wo0, %cc : index
      %wo2 = arith.muli %wo1, %R : index
      %wo3 = arith.addi %wo2, %rf : index
      %wo4 = arith.muli %wo3, %S : index
      %wo = arith.addi %wo4, %sf : index
      %scaled = arith.mulf %acc, %alpha : f32
      %no_old = arith.cmpf oeq, %beta, %f0 : f32
      %res = scf.if %no_old -> f32 {
        scf.yield %scaled : f32
      } else {
        %old_h = memref.load %Wm[%wo] : memref<?xf16, strided<[1]>, 1>
        %old = arith.extf %old_h : f16 to f32
        %ep_r = math.fma %old, %beta, %scaled : f32
        scf.yield %ep_r : f32
      }
      %res_h = arith.truncf %res : f32 to f16
      memref.store %res_h, %Wm[%wo] : memref<?xf16, strided<[1]>, 1>
    }
    gpu.return
  }

  // col[(c R S + r S + s) P Q + p Q + q] = x[n, c, p sh - ph + r' dh, q sw - pw + s' dw] (0 outside the image) for
  // image n, the stored filter tap (r, s) and its image tap (r', s') (flipped in convolution mode): the im2col matrix
  // of a forward convolution with one group, which then is y[n] (K x PQ) = w (K x C R S) col (C R S x PQ), a GEMM.
  // One thread per element of col.
  gpu.func @agx_conv2d_im2col_f32(%x0: memref<1xf32, 1>, %col0: memref<1xf32, 1>,
      %N_: i32, %C_: i32, %H_: i32, %W_: i32, %K_: i32, %R_: i32, %S_: i32, %P_: i32, %Q_: i32,
      %ph_: i32, %pw_: i32, %sh_: i32, %sw_: i32, %dh_: i32, %dw_: i32, %G_: i32, %flip_: i32,
      %xsn_: i32, %xsc_: i32, %xsh_: i32, %xsw_: i32, %n_: i32) kernel {
    %C = arith.index_cast %C_ : i32 to index
    %H = arith.index_cast %H_ : i32 to index
    %W = arith.index_cast %W_ : i32 to index
    %R = arith.index_cast %R_ : i32 to index
    %S = arith.index_cast %S_ : i32 to index
    %P = arith.index_cast %P_ : i32 to index
    %Q = arith.index_cast %Q_ : i32 to index
    %ph = arith.index_cast %ph_ : i32 to index
    %pw = arith.index_cast %pw_ : i32 to index
    %sh = arith.index_cast %sh_ : i32 to index
    %sw = arith.index_cast %sw_ : i32 to index
    %dh = arith.index_cast %dh_ : i32 to index
    %dw = arith.index_cast %dw_ : i32 to index
    %flip = arith.index_cast %flip_ : i32 to index
    %xsn = arith.index_cast %xsn_ : i32 to index
    %xsc = arith.index_cast %xsc_ : i32 to index
    %xsh = arith.index_cast %xsh_ : i32 to index
    %xsw = arith.index_cast %xsw_ : i32 to index
    %n = arith.index_cast %n_ : i32 to index
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %z = arith.constant 0.0 : f32
    %big = arith.constant 4611686018427387904 : index
    %Xm = memref.reinterpret_cast %x0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %Cm = memref.reinterpret_cast %col0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %id = gpu.global_id x
    %q = arith.remui %id, %Q : index
    %t1 = arith.divui %id, %Q : index
    %p = arith.remui %t1, %P : index
    %t2 = arith.divui %t1, %P : index
    %s = arith.remui %t2, %S : index
    %t3 = arith.divui %t2, %S : index
    %r = arith.remui %t3, %R : index
    %c = arith.divui %t3, %R : index
    %live = arith.cmpi ult, %c, %C : index
    scf.if %live {
      %no_flip = arith.cmpi eq, %flip, %i0 : index
      %Rm1 = arith.subi %R, %i1 : index
      %Sm1 = arith.subi %S, %i1 : index
      %rf = arith.subi %Rm1, %r : index
      %sf = arith.subi %Sm1, %s : index
      %ri = arith.select %no_flip, %r, %rf : index
      %si = arith.select %no_flip, %s, %sf : index
      %psh = arith.muli %p, %sh : index
      %rdh = arith.muli %ri, %dh : index
      %h1 = arith.addi %psh, %rdh : index
      %h = arith.subi %h1, %ph : index
      %qsw = arith.muli %q, %sw : index
      %sdw = arith.muli %si, %dw : index
      %w1 = arith.addi %qsw, %sdw : index
      %w = arith.subi %w1, %pw : index
      %h_ok0 = arith.cmpi sge, %h, %i0 : index
      %h_ok1 = arith.cmpi slt, %h, %H : index
      %w_ok0 = arith.cmpi sge, %w, %i0 : index
      %w_ok1 = arith.cmpi slt, %w, %W : index
      %ok0 = arith.andi %h_ok0, %h_ok1 : i1
      %ok1 = arith.andi %w_ok0, %w_ok1 : i1
      %ok = arith.andi %ok0, %ok1 : i1
      %v = scf.if %ok -> f32 {
        %xn = arith.muli %n, %xsn : index
        %xc = arith.muli %c, %xsc : index
        %xh = arith.muli %h, %xsh : index
        %xw = arith.muli %w, %xsw : index
        %xi0 = arith.addi %xn, %xc : index
        %xi1 = arith.addi %xi0, %xh : index
        %xi = arith.addi %xi1, %xw : index
        %xv = memref.load %Xm[%xi] : memref<?xf32, strided<[1]>, 1>
        scf.yield %xv : f32
      } else {
        scf.yield %z : f32
      }
      memref.store %v, %Cm[%id] : memref<?xf32, strided<[1]>, 1>
    }
    gpu.return
  }

  // col[(c R S + r S + s) P Q + p Q + q] = x[n, c, p sh - ph + r' dh, q sw - pw + s' dw] (0 outside the image) for
  // image n, the stored filter tap (r, s) and its image tap (r', s') (flipped in convolution mode): the im2col matrix
  // of a forward convolution with one group, which then is y[n] (K x PQ) = w (K x C R S) col (C R S x PQ), a GEMM.
  // One thread per element of col.
  gpu.func @agx_conv2d_im2col_f16(%x0: memref<1xf16, 1>, %col0: memref<1xf16, 1>,
      %N_: i32, %C_: i32, %H_: i32, %W_: i32, %K_: i32, %R_: i32, %S_: i32, %P_: i32, %Q_: i32,
      %ph_: i32, %pw_: i32, %sh_: i32, %sw_: i32, %dh_: i32, %dw_: i32, %G_: i32, %flip_: i32,
      %xsn_: i32, %xsc_: i32, %xsh_: i32, %xsw_: i32, %n_: i32) kernel {
    %C = arith.index_cast %C_ : i32 to index
    %H = arith.index_cast %H_ : i32 to index
    %W = arith.index_cast %W_ : i32 to index
    %R = arith.index_cast %R_ : i32 to index
    %S = arith.index_cast %S_ : i32 to index
    %P = arith.index_cast %P_ : i32 to index
    %Q = arith.index_cast %Q_ : i32 to index
    %ph = arith.index_cast %ph_ : i32 to index
    %pw = arith.index_cast %pw_ : i32 to index
    %sh = arith.index_cast %sh_ : i32 to index
    %sw = arith.index_cast %sw_ : i32 to index
    %dh = arith.index_cast %dh_ : i32 to index
    %dw = arith.index_cast %dw_ : i32 to index
    %flip = arith.index_cast %flip_ : i32 to index
    %xsn = arith.index_cast %xsn_ : i32 to index
    %xsc = arith.index_cast %xsc_ : i32 to index
    %xsh = arith.index_cast %xsh_ : i32 to index
    %xsw = arith.index_cast %xsw_ : i32 to index
    %n = arith.index_cast %n_ : i32 to index
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %z = arith.constant 0.0 : f16
    %big = arith.constant 4611686018427387904 : index
    %Xm = memref.reinterpret_cast %x0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf16, 1> to memref<?xf16, strided<[1]>, 1>
    %Cm = memref.reinterpret_cast %col0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf16, 1> to memref<?xf16, strided<[1]>, 1>
    %id = gpu.global_id x
    %q = arith.remui %id, %Q : index
    %t1 = arith.divui %id, %Q : index
    %p = arith.remui %t1, %P : index
    %t2 = arith.divui %t1, %P : index
    %s = arith.remui %t2, %S : index
    %t3 = arith.divui %t2, %S : index
    %r = arith.remui %t3, %R : index
    %c = arith.divui %t3, %R : index
    %live = arith.cmpi ult, %c, %C : index
    scf.if %live {
      %no_flip = arith.cmpi eq, %flip, %i0 : index
      %Rm1 = arith.subi %R, %i1 : index
      %Sm1 = arith.subi %S, %i1 : index
      %rf = arith.subi %Rm1, %r : index
      %sf = arith.subi %Sm1, %s : index
      %ri = arith.select %no_flip, %r, %rf : index
      %si = arith.select %no_flip, %s, %sf : index
      %psh = arith.muli %p, %sh : index
      %rdh = arith.muli %ri, %dh : index
      %h1 = arith.addi %psh, %rdh : index
      %h = arith.subi %h1, %ph : index
      %qsw = arith.muli %q, %sw : index
      %sdw = arith.muli %si, %dw : index
      %w1 = arith.addi %qsw, %sdw : index
      %w = arith.subi %w1, %pw : index
      %h_ok0 = arith.cmpi sge, %h, %i0 : index
      %h_ok1 = arith.cmpi slt, %h, %H : index
      %w_ok0 = arith.cmpi sge, %w, %i0 : index
      %w_ok1 = arith.cmpi slt, %w, %W : index
      %ok0 = arith.andi %h_ok0, %h_ok1 : i1
      %ok1 = arith.andi %w_ok0, %w_ok1 : i1
      %ok = arith.andi %ok0, %ok1 : i1
      %v = scf.if %ok -> f16 {
        %xn = arith.muli %n, %xsn : index
        %xc = arith.muli %c, %xsc : index
        %xh = arith.muli %h, %xsh : index
        %xw = arith.muli %w, %xsw : index
        %xi0 = arith.addi %xn, %xc : index
        %xi1 = arith.addi %xi0, %xh : index
        %xi = arith.addi %xi1, %xw : index
        %xv = memref.load %Xm[%xi] : memref<?xf16, strided<[1]>, 1>
        scf.yield %xv : f16
      } else {
        scf.yield %z : f16
      }
      memref.store %v, %Cm[%id] : memref<?xf16, strided<[1]>, 1>
    }
    gpu.return
  }
}
