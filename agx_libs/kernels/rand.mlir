// Random number kernels of zluda_rand_agx (cuRAND host API), for every AGX
// generation like sgemm.mlir. The generator is Philox4x32-10 (Salmon et al.,
// "Parallel random numbers: as easy as 1, 2, 3"): block b of the sequence is
// philox(counter (b, 0), key seed), four 32-bit words. The conversions to
// floats are cuRAND's documented ones: uniform values in (0, 1], normal
// values by the Box-Muller transform of pairs of uniforms.

gpu.module @agx_rand {
  // hi and lo words of a * b.
  func.func private @agx_rand_mulhilo(%a: i32, %b: i32) -> (i32, i32) {
    %c32 = arith.constant 32 : i64
    %a64 = arith.extui %a : i32 to i64
    %b64 = arith.extui %b : i32 to i64
    %p = arith.muli %a64, %b64 : i64
    %h64 = arith.shrui %p, %c32 : i64
    %h = arith.trunci %h64 : i64 to i32
    %l = arith.trunci %p : i64 to i32
    return %h, %l : i32, i32
  }

  // Philox4x32-10 of the counter (c0, c1, c2, c3) with the key (k0, k1).
  func.func private @agx_rand_philox(%c0: i32, %c1: i32, %c2: i32, %c3: i32, %k0: i32, %k1: i32)
      -> (i32, i32, i32, i32) {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %i10 = arith.constant 10 : index
    %m0 = arith.constant -766435501 : i32   // 0xD2511F53
    %m1 = arith.constant -845247145 : i32   // 0xCD9E8D57
    %w0 = arith.constant -1640531527 : i32  // 0x9E3779B9
    %w1 = arith.constant -1150833019 : i32  // 0xBB67AE85
    %r:6 = scf.for %round = %i0 to %i10 step %i1
        iter_args(%x0 = %c0, %x1 = %c1, %x2 = %c2, %x3 = %c3, %y0 = %k0, %y1 = %k1)
        -> (i32, i32, i32, i32, i32, i32) {
      %p0:2 = func.call @agx_rand_mulhilo(%m0, %x0) : (i32, i32) -> (i32, i32)
      %p1:2 = func.call @agx_rand_mulhilo(%m1, %x2) : (i32, i32) -> (i32, i32)
      %a = arith.xori %p1#0, %x1 : i32
      %n0 = arith.xori %a, %y0 : i32
      %b = arith.xori %p0#0, %x3 : i32
      %n2 = arith.xori %b, %y1 : i32
      %z0 = arith.addi %y0, %w0 : i32
      %z1 = arith.addi %y1, %w1 : i32
      scf.yield %n0, %p1#1, %n2, %p0#1, %z0, %z1 : i32, i32, i32, i32, i32, i32
    }
    return %r#0, %r#1, %r#2, %r#3 : i32, i32, i32, i32
  }

  // The four words of block b.
  func.func private @agx_rand_block(%b: i64, %seed: i64) -> (i32, i32, i32, i32) {
    %c32 = arith.constant 32 : i64
    %z = arith.constant 0 : i32
    %blo = arith.trunci %b : i64 to i32
    %bh = arith.shrui %b, %c32 : i64
    %bhi = arith.trunci %bh : i64 to i32
    %klo = arith.trunci %seed : i64 to i32
    %kh = arith.shrui %seed, %c32 : i64
    %khi = arith.trunci %kh : i64 to i32
    %r:4 = func.call @agx_rand_philox(%blo, %bhi, %z, %z, %klo, %khi)
        : (i32, i32, i32, i32, i32, i32) -> (i32, i32, i32, i32)
    return %r#0, %r#1, %r#2, %r#3 : i32, i32, i32, i32
  }

  // x * 2^-32 + 2^-33, in (0, 1].
  func.func private @agx_rand_u01(%x: i32) -> f32 {
    %inv = arith.constant 2.3283064365386963e-10 : f32
    %half = arith.constant 1.1641532182693481e-10 : f32
    %f = arith.uitofp %x : i32 to f32
    %u = math.fma %f, %inv, %half : f32
    return %u : f32
  }

  // out[j] for j < n, from the blocks start, start + 1, ... (four values per
  // block and thread):
  //   mode 0: the words
  //   mode 1: uniform in (0, 1]
  //   mode 2: normal with mean p0 and standard deviation p1 (Box-Muller of
  //           the word pairs)
  //   mode 3: exp of mode 2 (log-normal)
  gpu.func @agx_rand32(%out0: memref<1xi32, 1>, %n_: i64, %start: i64, %seed: i64, %mode_: i64,
                       %p0: f32, %p1: f32) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %i2 = arith.constant 2 : index
    %i3 = arith.constant 3 : index
    %i4 = arith.constant 4 : index
    %big = arith.constant 4611686018427387904 : index
    %c1 = arith.constant 1 : i64
    %c2 = arith.constant 2 : i64
    %c3 = arith.constant 3 : i64
    %m2 = arith.constant -2.0 : f32
    %inv2pi = arith.constant 1.4629180792671596e-09 : f32  // 2^-32 2 pi
    %half2pi = arith.constant 7.314590396335798e-10 : f32
    %n = arith.index_cast %n_ : i64 to index
    %out = memref.reinterpret_cast %out0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xi32, 1> to memref<?xi32, strided<[1]>, 1>
    %t = gpu.global_id x
    %j0 = arith.muli %t, %i4 : index
    %live = arith.cmpi ult, %j0, %n : index
    scf.if %live {
      %t64 = arith.index_cast %t : index to i64
      %b = arith.addi %start, %t64 : i64
      %w:4 = func.call @agx_rand_block(%b, %seed) : (i64, i64) -> (i32, i32, i32, i32)
      %is_u = arith.cmpi eq, %mode_, %c1 : i64
      %is_raw = arith.cmpi ult, %mode_, %c1 : i64
      %is_log = arith.cmpi eq, %mode_, %c3 : i64
      %r:4 = scf.if %is_raw -> (i32, i32, i32, i32) {
        scf.yield %w#0, %w#1, %w#2, %w#3 : i32, i32, i32, i32
      } else {
        %f:4 = scf.if %is_u -> (f32, f32, f32, f32) {
          %u0 = func.call @agx_rand_u01(%w#0) : (i32) -> f32
          %u1 = func.call @agx_rand_u01(%w#1) : (i32) -> f32
          %u2 = func.call @agx_rand_u01(%w#2) : (i32) -> f32
          %u3 = func.call @agx_rand_u01(%w#3) : (i32) -> f32
          scf.yield %u0, %u1, %u2, %u3 : f32, f32, f32, f32
        } else {
          %u0 = func.call @agx_rand_u01(%w#0) : (i32) -> f32
          %u2 = func.call @agx_rand_u01(%w#2) : (i32) -> f32
          %v1f = arith.uitofp %w#1 : i32 to f32
          %v1 = math.fma %v1f, %inv2pi, %half2pi : f32
          %v3f = arith.uitofp %w#3 : i32 to f32
          %v3 = math.fma %v3f, %inv2pi, %half2pi : f32
          %l0 = math.log %u0 : f32
          %l2 = math.log %u2 : f32
          %q0 = arith.mulf %l0, %m2 : f32
          %q2 = arith.mulf %l2, %m2 : f32
          %s0 = math.sqrt %q0 : f32
          %s2 = math.sqrt %q2 : f32
          %sn1 = math.sin %v1 : f32
          %cs1 = math.cos %v1 : f32
          %sn3 = math.sin %v3 : f32
          %cs3 = math.cos %v3 : f32
          %z0 = arith.mulf %s0, %sn1 : f32
          %z1 = arith.mulf %s0, %cs1 : f32
          %z2 = arith.mulf %s2, %sn3 : f32
          %z3 = arith.mulf %s2, %cs3 : f32
          %y0 = math.fma %z0, %p1, %p0 : f32
          %y1 = math.fma %z1, %p1, %p0 : f32
          %y2 = math.fma %z2, %p1, %p0 : f32
          %y3 = math.fma %z3, %p1, %p0 : f32
          %e:4 = scf.if %is_log -> (f32, f32, f32, f32) {
            %e0 = math.exp %y0 : f32
            %e1 = math.exp %y1 : f32
            %e2 = math.exp %y2 : f32
            %e3 = math.exp %y3 : f32
            scf.yield %e0, %e1, %e2, %e3 : f32, f32, f32, f32
          } else {
            scf.yield %y0, %y1, %y2, %y3 : f32, f32, f32, f32
          }
          scf.yield %e#0, %e#1, %e#2, %e#3 : f32, f32, f32, f32
        }
        %b0 = arith.bitcast %f#0 : f32 to i32
        %b1 = arith.bitcast %f#1 : f32 to i32
        %b2 = arith.bitcast %f#2 : f32 to i32
        %b3 = arith.bitcast %f#3 : f32 to i32
        scf.yield %b0, %b1, %b2, %b3 : i32, i32, i32, i32
      }
      memref.store %r#0, %out[%j0] : memref<?xi32, strided<[1]>, 1>
      %j1 = arith.addi %j0, %i1 : index
      %l1 = arith.cmpi ult, %j1, %n : index
      scf.if %l1 {
        memref.store %r#1, %out[%j1] : memref<?xi32, strided<[1]>, 1>
      }
      %j2 = arith.addi %j0, %i2 : index
      %l2 = arith.cmpi ult, %j2, %n : index
      scf.if %l2 {
        memref.store %r#2, %out[%j2] : memref<?xi32, strided<[1]>, 1>
      }
      %j3 = arith.addi %j0, %i3 : index
      %l3 = arith.cmpi ult, %j3, %n : index
      scf.if %l3 {
        memref.store %r#3, %out[%j3] : memref<?xi32, strided<[1]>, 1>
      }
    }
    gpu.return
  }

  // (x ^ (y << 21)) 2^-53 + 2^-54, in (0, 1] with 53 random bits.
  func.func private @agx_rand_u01_f64(%x: i32, %y: i32) -> f64 {
    %c21 = arith.constant 21 : i64
    %inv = arith.constant 1.1102230246251565e-16 : f64
    %half = arith.constant 5.551115123125783e-17 : f64
    %x64 = arith.extui %x : i32 to i64
    %y64 = arith.extui %y : i32 to i64
    %ys = arith.shli %y64, %c21 : i64
    %z = arith.xori %x64, %ys : i64
    %f = arith.uitofp %z : i64 to f64
    %u = math.fma %f, %inv, %half : f64
    return %u : f64
  }

  // log(u) for normal u > 0: u = m 2^e with m in [sqrt(1/2), sqrt(2)),
  // log m = 2 atanh(s) with s = (m - 1) / (m + 1), |s| <= 0.1716.
  func.func private @agx_rand_log_f64(%u: f64) -> f64 {
    %c52 = arith.constant 52 : i64
    %e_mask = arith.constant 2047 : i64
    %bias = arith.constant 1023 : i64
    %frac = arith.constant 4503599627370495 : i64
    %one_e = arith.constant 4607182418800017408 : i64      // exponent of 1.0
    %half_e = arith.constant 4602678819172646912 : i64     // exponent of 0.5
    %sqrt2 = arith.constant 1.4142135623730951 : f64
    %f1 = arith.constant 1.0 : f64
    %f2 = arith.constant 2.0 : f64
    %ln2hi = arith.constant 0.6931471803691238 : f64
    %ln2lo = arith.constant 1.9082149292705877e-10 : f64
    %bits = arith.bitcast %u : f64 to i64
    %es = arith.shrui %bits, %c52 : i64
    %eb = arith.andi %es, %e_mask : i64
    %e0 = arith.subi %eb, %bias : i64
    %fr = arith.andi %bits, %frac : i64
    %mb = arith.ori %fr, %one_e : i64
    %m1 = arith.bitcast %mb : i64 to f64
    %big = arith.cmpf oge, %m1, %sqrt2 : f64
    %mb2 = arith.ori %fr, %half_e : i64
    %m2 = arith.bitcast %mb2 : i64 to f64
    %m = arith.select %big, %m2, %m1 : f64
    %c1 = arith.constant 1 : i64
    %e1 = arith.addi %e0, %c1 : i64
    %e = arith.select %big, %e1, %e0 : i64
    %num = arith.subf %m, %f1 : f64
    %den = arith.addf %m, %f1 : f64
    %s = arith.divf %num, %den : f64
    %s2 = arith.mulf %s, %s : f64
    // 1 + s2/3 + s2^2/5 + ... + s2^10/21 (Horner)
    %a21 = arith.constant 0.047619047619047616 : f64
    %a19 = arith.constant 0.05263157894736842 : f64
    %a17 = arith.constant 0.058823529411764705 : f64
    %a15 = arith.constant 0.06666666666666667 : f64
    %a13 = arith.constant 0.07692307692307693 : f64
    %a11 = arith.constant 0.09090909090909091 : f64
    %a9 = arith.constant 0.1111111111111111 : f64
    %a7 = arith.constant 0.14285714285714285 : f64
    %a5 = arith.constant 0.2 : f64
    %a3 = arith.constant 0.3333333333333333 : f64
    %h1 = math.fma %a21, %s2, %a19 : f64
    %h2 = math.fma %h1, %s2, %a17 : f64
    %h3 = math.fma %h2, %s2, %a15 : f64
    %h4 = math.fma %h3, %s2, %a13 : f64
    %h5 = math.fma %h4, %s2, %a11 : f64
    %h6 = math.fma %h5, %s2, %a9 : f64
    %h7 = math.fma %h6, %s2, %a7 : f64
    %h8 = math.fma %h7, %s2, %a5 : f64
    %h9 = math.fma %h8, %s2, %a3 : f64
    %h10 = math.fma %h9, %s2, %f1 : f64
    %s_2 = arith.mulf %s, %f2 : f64
    %lm = arith.mulf %s_2, %h10 : f64
    %ef = arith.sitofp %e : i64 to f64
    %lo = math.fma %ef, %ln2lo, %lm : f64
    %r = math.fma %ef, %ln2hi, %lo : f64
    return %r : f64
  }

  // (sin, cos) of 2 pi t for t in [0, 1]: t = (k + d) / 4 with k = round(4 t)
  // and |d| <= 1/2 (exact), x = d pi / 2 in [-pi/4, pi/4].
  func.func private @agx_rand_sincos2pi_f64(%t: f64) -> (f64, f64) {
    %f4 = arith.constant 4.0 : f64
    %pi_2 = arith.constant 1.5707963267948966 : f64
    %f1 = arith.constant 1.0 : f64
    %c1 = arith.constant 1 : i64
    %c2 = arith.constant 2 : i64
    %c3 = arith.constant 3 : i64
    %r = arith.mulf %t, %f4 : f64
    %kf = math.roundeven %r : f64
    %d = arith.subf %r, %kf : f64
    %x = arith.mulf %d, %pi_2 : f64
    %x2 = arith.mulf %x, %x : f64
    // sin x = x (1 - x^2/3! + ... - x^16/17!), cos x = 1 - x^2/2! + ... - x^18/18!
    %s17 = arith.constant 2.8114572543455206e-15 : f64
    %s15 = arith.constant -7.647163731819816e-13 : f64
    %s13 = arith.constant 1.6059043836821613e-10 : f64
    %s11 = arith.constant -2.505210838544172e-08 : f64
    %s9 = arith.constant 2.7557319223985893e-06 : f64
    %s7 = arith.constant -0.0001984126984126984 : f64
    %s5 = arith.constant 0.008333333333333333 : f64
    %s3 = arith.constant -0.16666666666666666 : f64
    %ps1 = math.fma %s17, %x2, %s15 : f64
    %ps2 = math.fma %ps1, %x2, %s13 : f64
    %ps3 = math.fma %ps2, %x2, %s11 : f64
    %ps4 = math.fma %ps3, %x2, %s9 : f64
    %ps5 = math.fma %ps4, %x2, %s7 : f64
    %ps6 = math.fma %ps5, %x2, %s5 : f64
    %ps7 = math.fma %ps6, %x2, %s3 : f64
    %x3 = arith.mulf %x2, %x : f64
    %sn = math.fma %ps7, %x3, %x : f64
    %c18 = arith.constant -1.5619206968586225e-16 : f64
    %c16 = arith.constant 4.779477332387385e-14 : f64
    %c14 = arith.constant -1.1470745597729725e-11 : f64
    %c12 = arith.constant 2.08767569878681e-09 : f64
    %c10 = arith.constant -2.755731922398589e-07 : f64
    %c8 = arith.constant 2.48015873015873e-05 : f64
    %c6 = arith.constant -0.001388888888888889 : f64
    %c4 = arith.constant 0.041666666666666664 : f64
    %c2f = arith.constant 0.5 : f64
    %pc1 = math.fma %c18, %x2, %c16 : f64
    %pc2 = math.fma %pc1, %x2, %c14 : f64
    %pc3 = math.fma %pc2, %x2, %c12 : f64
    %pc4 = math.fma %pc3, %x2, %c10 : f64
    %pc5 = math.fma %pc4, %x2, %c8 : f64
    %pc6 = math.fma %pc5, %x2, %c6 : f64
    %pc7 = math.fma %pc6, %x2, %c4 : f64
    %pc8 = arith.mulf %pc7, %x2 : f64
    %pc9 = arith.subf %pc8, %c2f : f64
    %cs = math.fma %pc9, %x2, %f1 : f64
    %k64 = arith.fptosi %kf : f64 to i64
    %q = arith.andi %k64, %c3 : i64
    %nsn = arith.negf %sn : f64
    %ncs = arith.negf %cs : f64
    %q1 = arith.cmpi eq, %q, %c1 : i64
    %q2 = arith.cmpi eq, %q, %c2 : i64
    %q3 = arith.cmpi eq, %q, %c3 : i64
    // quadrant k: sin(k pi/2 + x), cos(k pi/2 + x)
    %sa = arith.select %q1, %cs, %sn : f64
    %sb = arith.select %q2, %nsn, %sa : f64
    %so = arith.select %q3, %ncs, %sb : f64
    %ca = arith.select %q1, %nsn, %cs : f64
    %cb = arith.select %q2, %ncs, %ca : f64
    %co = arith.select %q3, %sn, %cb : f64
    return %so, %co : f64, f64
  }

  // The f64 forms of agx_rand32 modes 1 to 3, two values per block (two
  // words per uniform; a normal pair from the four words).
  gpu.func @agx_rand64(%out0: memref<1xf64, 1>, %n_: i64, %start: i64, %seed: i64, %mode_: i64,
                       %p0: f64, %p1: f64) kernel {
    %i1 = arith.constant 1 : index
    %i2 = arith.constant 2 : index
    %big = arith.constant 4611686018427387904 : index
    %c1 = arith.constant 1 : i64
    %c3 = arith.constant 3 : i64
    %m2 = arith.constant -2.0 : f64
    %n = arith.index_cast %n_ : i64 to index
    %out = memref.reinterpret_cast %out0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf64, 1> to memref<?xf64, strided<[1]>, 1>
    %t = gpu.global_id x
    %j0 = arith.muli %t, %i2 : index
    %live = arith.cmpi ult, %j0, %n : index
    scf.if %live {
      %t64 = arith.index_cast %t : index to i64
      %b = arith.addi %start, %t64 : i64
      %w:4 = func.call @agx_rand_block(%b, %seed) : (i64, i64) -> (i32, i32, i32, i32)
      %u0 = func.call @agx_rand_u01_f64(%w#0, %w#1) : (i32, i32) -> f64
      %u1 = func.call @agx_rand_u01_f64(%w#2, %w#3) : (i32, i32) -> f64
      %is_u = arith.cmpi eq, %mode_, %c1 : i64
      %is_log = arith.cmpi eq, %mode_, %c3 : i64
      %r:2 = scf.if %is_u -> (f64, f64) {
        scf.yield %u0, %u1 : f64, f64
      } else {
        %l = func.call @agx_rand_log_f64(%u0) : (f64) -> f64
        %q = arith.mulf %l, %m2 : f64
        %s = math.sqrt %q : f64
        %sc:2 = func.call @agx_rand_sincos2pi_f64(%u1) : (f64) -> (f64, f64)
        %z0 = arith.mulf %s, %sc#0 : f64
        %z1 = arith.mulf %s, %sc#1 : f64
        %y0 = math.fma %z0, %p1, %p0 : f64
        %y1 = math.fma %z1, %p1, %p0 : f64
        %e:2 = scf.if %is_log -> (f64, f64) {
          %e0 = func.call @agx_rand_exp_f64(%y0) : (f64) -> f64
          %e1 = func.call @agx_rand_exp_f64(%y1) : (f64) -> f64
          scf.yield %e0, %e1 : f64, f64
        } else {
          scf.yield %y0, %y1 : f64, f64
        }
        scf.yield %e#0, %e#1 : f64, f64
      }
      memref.store %r#0, %out[%j0] : memref<?xf64, strided<[1]>, 1>
      %j1 = arith.addi %j0, %i1 : index
      %l1 = arith.cmpi ult, %j1, %n : index
      scf.if %l1 {
        memref.store %r#1, %out[%j1] : memref<?xf64, strided<[1]>, 1>
      }
    }
    gpu.return
  }

  // exp(y) = 2^k exp(r) with k = round(y / ln 2) and |r| <= ln 2 / 2; 0 below
  // 2^-1022 and inf above the f64 range.
  func.func private @agx_rand_exp_f64(%y: f64) -> f64 {
    %inv_ln2 = arith.constant 1.4426950408889634 : f64
    %ln2hi = arith.constant 0.6931471803691238 : f64
    %ln2lo = arith.constant 1.9082149292705877e-10 : f64
    %f0 = arith.constant 0.0 : f64
    %f1 = arith.constant 1.0 : f64
    %inf = arith.constant 0x7FF0000000000000 : f64
    %hi = arith.constant 709.0 : f64
    %lo = arith.constant -708.0 : f64
    %c52 = arith.constant 52 : i64
    %bias = arith.constant 1023 : i64
    %q = arith.mulf %y, %inv_ln2 : f64
    %kf = math.roundeven %q : f64
    %nk = arith.negf %kf : f64
    %r1 = math.fma %nk, %ln2hi, %y : f64
    %r = math.fma %nk, %ln2lo, %r1 : f64
    // 1 + r + r^2/2! + ... + r^13/13! (Horner)
    %a13 = arith.constant 1.6059043836821613e-10 : f64
    %a12 = arith.constant 2.08767569878681e-09 : f64
    %a11 = arith.constant 2.505210838544172e-08 : f64
    %a10 = arith.constant 2.755731922398589e-07 : f64
    %a9 = arith.constant 2.7557319223985893e-06 : f64
    %a8 = arith.constant 2.48015873015873e-05 : f64
    %a7 = arith.constant 0.0001984126984126984 : f64
    %a6 = arith.constant 0.001388888888888889 : f64
    %a5 = arith.constant 0.008333333333333333 : f64
    %a4 = arith.constant 0.041666666666666664 : f64
    %a3 = arith.constant 0.16666666666666666 : f64
    %a2 = arith.constant 0.5 : f64
    %h1 = math.fma %a13, %r, %a12 : f64
    %h2 = math.fma %h1, %r, %a11 : f64
    %h3 = math.fma %h2, %r, %a10 : f64
    %h4 = math.fma %h3, %r, %a9 : f64
    %h5 = math.fma %h4, %r, %a8 : f64
    %h6 = math.fma %h5, %r, %a7 : f64
    %h7 = math.fma %h6, %r, %a6 : f64
    %h8 = math.fma %h7, %r, %a5 : f64
    %h9 = math.fma %h8, %r, %a4 : f64
    %h10 = math.fma %h9, %r, %a3 : f64
    %h11 = math.fma %h10, %r, %a2 : f64
    %h12 = math.fma %h11, %r, %f1 : f64
    %er = math.fma %h12, %r, %f1 : f64
    %k = arith.fptosi %kf : f64 to i64
    %kb = arith.addi %k, %bias : i64
    %pb = arith.shli %kb, %c52 : i64
    %p = arith.bitcast %pb : i64 to f64
    %v = arith.mulf %er, %p : f64
    %over = arith.cmpf ogt, %y, %hi : f64
    %under = arith.cmpf olt, %y, %lo : f64
    %v1 = arith.select %over, %inf, %v : f64
    %v2 = arith.select %under, %f0, %v1 : f64
    return %v2 : f64
  }

  // Poisson values out[j] (j < n) with mean lambda, value j from its own
  // stream: blocks philox(counter (start + j, iteration, 1), key seed).
  // lambda < 64: the number of exponential arrivals (-log u) before lambda;
  // larger: the normal approximation round(lambda + sqrt(lambda) z), z from
  // the first block, at least 0.
  gpu.func @agx_rand_poisson(%out0: memref<1xi32, 1>, %n_: i64, %start: i64, %seed: i64, %lambda: f32) kernel {
    %big = arith.constant 4611686018427387904 : index
    %c32 = arith.constant 32 : i64
    %z = arith.constant 0 : i32
    %one = arith.constant 1 : i32
    %f0 = arith.constant 0.0 : f32
    %fhalf = arith.constant 0.5 : f32
    %m2 = arith.constant -2.0 : f32
    %limit = arith.constant 64.0 : f32
    %inv2pi = arith.constant 1.4629180792671596e-09 : f32
    %half2pi = arith.constant 7.314590396335798e-10 : f32
    %false = arith.constant false
    %true = arith.constant true
    %n = arith.index_cast %n_ : i64 to index
    %out = memref.reinterpret_cast %out0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xi32, 1> to memref<?xi32, strided<[1]>, 1>
    %j = gpu.global_id x
    %live = arith.cmpi ult, %j, %n : index
    scf.if %live {
      %j64 = arith.index_cast %j : index to i64
      %g = arith.addi %start, %j64 : i64
      %glo = arith.trunci %g : i64 to i32
      %gh = arith.shrui %g, %c32 : i64
      %ghi = arith.trunci %gh : i64 to i32
      %klo = arith.trunci %seed : i64 to i32
      %kh = arith.shrui %seed, %c32 : i64
      %khi = arith.trunci %kh : i64 to i32
      %small = arith.cmpf olt, %lambda, %limit : f32
      %k = scf.if %small -> i32 {
        %res:4 = scf.while (%it = %z, %sum = %f0, %cnt = %z, %done = %false) : (i32, f32, i32, i1) -> (i32, f32, i32, i1) {
          %go = arith.xori %done, %true : i1
          scf.condition(%go) %it, %sum, %cnt, %done : i32, f32, i32, i1
        } do {
        ^bb0(%it1: i32, %sum1: f32, %cnt1: i32, %done1: i1):
          %w:4 = func.call @agx_rand_philox(%glo, %ghi, %it1, %one, %klo, %khi)
              : (i32, i32, i32, i32, i32, i32) -> (i32, i32, i32, i32)
          %s0:3 = func.call @agx_rand_arrival(%w#0, %sum1, %cnt1, %done1, %lambda) : (i32, f32, i32, i1, f32) -> (f32, i32, i1)
          %s1:3 = func.call @agx_rand_arrival(%w#1, %s0#0, %s0#1, %s0#2, %lambda) : (i32, f32, i32, i1, f32) -> (f32, i32, i1)
          %s2:3 = func.call @agx_rand_arrival(%w#2, %s1#0, %s1#1, %s1#2, %lambda) : (i32, f32, i32, i1, f32) -> (f32, i32, i1)
          %s3:3 = func.call @agx_rand_arrival(%w#3, %s2#0, %s2#1, %s2#2, %lambda) : (i32, f32, i32, i1, f32) -> (f32, i32, i1)
          %it2 = arith.addi %it1, %one : i32
          scf.yield %it2, %s3#0, %s3#1, %s3#2 : i32, f32, i32, i1
        }
        scf.yield %res#2 : i32
      } else {
        %w:4 = func.call @agx_rand_philox(%glo, %ghi, %z, %one, %klo, %khi)
            : (i32, i32, i32, i32, i32, i32) -> (i32, i32, i32, i32)
        %u = func.call @agx_rand_u01(%w#0) : (i32) -> f32
        %vf = arith.uitofp %w#1 : i32 to f32
        %v = math.fma %vf, %inv2pi, %half2pi : f32
        %l = math.log %u : f32
        %q = arith.mulf %l, %m2 : f32
        %s = math.sqrt %q : f32
        %sn = math.sin %v : f32
        %zn = arith.mulf %s, %sn : f32
        %sl = math.sqrt %lambda : f32
        %x = math.fma %sl, %zn, %lambda : f32
        %xr = arith.addf %x, %fhalf : f32
        %xf = math.floor %xr : f32
        %xc = arith.maximumf %xf, %f0 : f32
        %ki = arith.fptoui %xc : f32 to i32
        scf.yield %ki : i32
      }
      memref.store %k, %out[%j] : memref<?xi32, strided<[1]>, 1>
    }
    gpu.return
  }

  // One arrival of agx_rand_poisson: sum += -log u; past lambda it is done,
  // else one more event.
  func.func private @agx_rand_arrival(%w: i32, %sum: f32, %cnt: i32, %done: i1, %lambda: f32) -> (f32, i32, i1) {
    %one = arith.constant 1 : i32
    %u = func.call @agx_rand_u01(%w) : (i32) -> f32
    %l = math.log %u : f32
    %s = arith.subf %sum, %l : f32
    %past = arith.cmpf ogt, %s, %lambda : f32
    %c1 = arith.addi %cnt, %one : i32
    %c = arith.select %past, %cnt, %c1 : i32
    %d = arith.ori %done, %past : i1
    %s2 = arith.select %done, %sum, %s : f32
    %c2 = arith.select %done, %cnt, %c : i32
    return %s2, %c2, %d : f32, i32, i1
  }
}
