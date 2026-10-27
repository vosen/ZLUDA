// FFT kernels of zluda_fft_agx (cuFFT), single precision, for every AGX
// generation like sgemm.mlir. Complex numbers are interleaved (re, im) f32
// pairs. A transform of length n runs over `batch` sequences: element i of
// sequence b is at offset b * dist + i * stride (in complex elements). tw holds
// the n twiddles exp(-2 pi i t / n), t = 0 ... n - 1, computed on the host in
// double precision; inverse transforms use their conjugates. Like cuFFT, the
// inverse transform is not normalized.

gpu.module @agx_fft {
  // One radix-2 pass of the Stockham autosort FFT for n a power of two: with
  // span ns (1, 2, 4, ..., n / 2), butterfly j of n / 2 reads src[j] and
  // src[j + n / 2] and writes dst[(j / ns) 2 ns + j mod ns] and that + ns.
  // log2(n) passes, alternating two buffers, give the transform in order.
  // Launch an (n / 2) x batch grid.
  gpu.func @agx_fft_radix2(%src0: memref<1xf32, 1>, %dst0: memref<1xf32, 1>,
                           %tw0: memref<1xf32, 1>, %n_: i32, %ns_: i32,
                           %batch_: i32, %stride_: i32, %dist_: i32,
                           %inverse_: i32) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %i2 = arith.constant 2 : index
    %big = arith.constant 4611686018427387904 : index
    %n = arith.index_cast %n_ : i32 to index
    %ns = arith.index_cast %ns_ : i32 to index
    %batch = arith.index_cast %batch_ : i32 to index
    %stride = arith.index_cast %stride_ : i32 to index
    %dist = arith.index_cast %dist_ : i32 to index
    %inverse = arith.index_cast %inverse_ : i32 to index
    %Src = memref.reinterpret_cast %src0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %Dst = memref.reinterpret_cast %dst0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %Tw = memref.reinterpret_cast %tw0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %half = arith.divui %n, %i2 : index
    %j = gpu.global_id x
    %b = gpu.global_id y
    %in_j = arith.cmpi ult, %j, %half : index
    %in_b = arith.cmpi ult, %b, %batch : index
    %live = arith.andi %in_j, %in_b : i1
    scf.if %live {
      %base = arith.muli %b, %dist : index
      // element offsets in floats: 2 * (base + i * stride)
      %ia0 = arith.muli %j, %stride : index
      %ia1 = arith.addi %base, %ia0 : index
      %ia = arith.muli %ia1, %i2 : index
      %jh = arith.addi %j, %half : index
      %ib0 = arith.muli %jh, %stride : index
      %ib1 = arith.addi %base, %ib0 : index
      %ib = arith.muli %ib1, %i2 : index
      %ia_im = arith.addi %ia, %i1 : index
      %ib_im = arith.addi %ib, %i1 : index
      %ar = memref.load %Src[%ia] : memref<?xf32, strided<[1]>, 1>
      %ai = memref.load %Src[%ia_im] : memref<?xf32, strided<[1]>, 1>
      %br = memref.load %Src[%ib] : memref<?xf32, strided<[1]>, 1>
      %bi = memref.load %Src[%ib_im] : memref<?xf32, strided<[1]>, 1>
      // w = exp(-+ 2 pi i k / (2 ns)) = tw[k n / (2 ns)]
      %k = arith.remui %j, %ns : index
      %ns2 = arith.muli %ns, %i2 : index
      %step = arith.divui %n, %ns2 : index
      %t = arith.muli %k, %step : index
      %t2 = arith.muli %t, %i2 : index
      %t2i = arith.addi %t2, %i1 : index
      %wr = memref.load %Tw[%t2] : memref<?xf32, strided<[1]>, 1>
      %wi0 = memref.load %Tw[%t2i] : memref<?xf32, strided<[1]>, 1>
      %is_inv = arith.cmpi ne, %inverse, %i0 : index
      %wi_neg = arith.negf %wi0 : f32
      %wi = arith.select %is_inv, %wi_neg, %wi0 : f32
      // v = b * w
      %bwr0 = arith.mulf %br, %wr : f32
      %vr2 = arith.mulf %bi, %wi : f32
      %vr_ok = arith.subf %bwr0, %vr2 : f32
      %bwi0 = arith.mulf %br, %wi : f32
      %vi = math.fma %bi, %wr, %bwi0 : f32
      %sr = arith.addf %ar, %vr_ok : f32
      %si = arith.addf %ai, %vi : f32
      %dr = arith.subf %ar, %vr_ok : f32
      %di = arith.subf %ai, %vi : f32
      %jd0 = arith.divui %j, %ns : index
      %jd1 = arith.muli %jd0, %ns2 : index
      %jd = arith.addi %jd1, %k : index
      %oa0 = arith.muli %jd, %stride : index
      %oa1 = arith.addi %base, %oa0 : index
      %oa = arith.muli %oa1, %i2 : index
      %jdn = arith.addi %jd, %ns : index
      %ob0 = arith.muli %jdn, %stride : index
      %ob1 = arith.addi %base, %ob0 : index
      %ob = arith.muli %ob1, %i2 : index
      %oa_im = arith.addi %oa, %i1 : index
      %ob_im = arith.addi %ob, %i1 : index
      memref.store %sr, %Dst[%oa] : memref<?xf32, strided<[1]>, 1>
      memref.store %si, %Dst[%oa_im] : memref<?xf32, strided<[1]>, 1>
      memref.store %dr, %Dst[%ob] : memref<?xf32, strided<[1]>, 1>
      memref.store %di, %Dst[%ob_im] : memref<?xf32, strided<[1]>, 1>
    }
    gpu.return
  }

  // One radix-4 pass of the Stockham FFT (two radix-2 passes in one, half the
  // memory traffic): with span ns (n / ns a multiple of 4), butterfly j of n / 4
  // reads x_k = src[j + k n / 4], k = 0 ... 3, and with q = j mod ns and
  // t_k = x_k w^(q k), w = exp(-+ 2 pi i / (4 ns)) = tw[n / (4 ns)], writes the
  // 4-point DFT of t to dst[(j / ns) 4 ns + q + m ns], m = 0 ... 3. Launch an
  // (n / 4) x batch grid.
  gpu.func @agx_fft_radix4(%src0: memref<1xf32, 1>, %dst0: memref<1xf32, 1>,
                           %tw0: memref<1xf32, 1>, %n_: i32, %ns_: i32,
                           %batch_: i32, %stride_: i32, %dist_: i32,
                           %inverse_: i32) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %i2 = arith.constant 2 : index
    %i4 = arith.constant 4 : index
    %big = arith.constant 4611686018427387904 : index
    %n = arith.index_cast %n_ : i32 to index
    %ns = arith.index_cast %ns_ : i32 to index
    %batch = arith.index_cast %batch_ : i32 to index
    %stride = arith.index_cast %stride_ : i32 to index
    %dist = arith.index_cast %dist_ : i32 to index
    %inverse = arith.index_cast %inverse_ : i32 to index
    %Src = memref.reinterpret_cast %src0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %Dst = memref.reinterpret_cast %dst0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %Tw = memref.reinterpret_cast %tw0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %quarter = arith.divui %n, %i4 : index
    %j = gpu.global_id x
    %b = gpu.global_id y
    %in_j = arith.cmpi ult, %j, %quarter : index
    %in_b = arith.cmpi ult, %b, %batch : index
    %live = arith.andi %in_j, %in_b : i1
    scf.if %live {
      %base = arith.muli %b, %dist : index
      %q = arith.remui %j, %ns : index
      %ns4 = arith.muli %ns, %i4 : index
      %step = arith.divui %n, %ns4 : index
      %qs = arith.muli %q, %step : index
      %is_inv = arith.cmpi ne, %inverse, %i0 : index
      %c0 = arith.constant 0 : index
      %off00 = arith.muli %quarter, %c0 : index
      %jj0 = arith.addi %j, %off00 : index
      %e00 = arith.muli %jj0, %stride : index
      %e01 = arith.addi %base, %e00 : index
      %e0 = arith.muli %e01, %i2 : index
      %e0i = arith.addi %e0, %i1 : index
      %xr0 = memref.load %Src[%e0] : memref<?xf32, strided<[1]>, 1>
      %xi0 = memref.load %Src[%e0i] : memref<?xf32, strided<[1]>, 1>
      %c1 = arith.constant 1 : index
      %off10 = arith.muli %quarter, %c1 : index
      %jj1 = arith.addi %j, %off10 : index
      %e10 = arith.muli %jj1, %stride : index
      %e11 = arith.addi %base, %e10 : index
      %e1 = arith.muli %e11, %i2 : index
      %e1i = arith.addi %e1, %i1 : index
      %xr1 = memref.load %Src[%e1] : memref<?xf32, strided<[1]>, 1>
      %xi1 = memref.load %Src[%e1i] : memref<?xf32, strided<[1]>, 1>
      %tq1 = arith.muli %qs, %c1 : index
      %tq12 = arith.muli %tq1, %i2 : index
      %tq12i = arith.addi %tq12, %i1 : index
      %wr1 = memref.load %Tw[%tq12] : memref<?xf32, strided<[1]>, 1>
      %wi10 = memref.load %Tw[%tq12i] : memref<?xf32, strided<[1]>, 1>
      %wi1n = arith.negf %wi10 : f32
      %wi1 = arith.select %is_inv, %wi1n, %wi10 : f32
      %a1r0 = arith.mulf %xr1, %wr1 : f32
      %a1r1 = arith.mulf %xi1, %wi1 : f32
      %tr1 = arith.subf %a1r0, %a1r1 : f32
      %a1i0 = arith.mulf %xr1, %wi1 : f32
      %ti1 = math.fma %xi1, %wr1, %a1i0 : f32
      %c2 = arith.constant 2 : index
      %off20 = arith.muli %quarter, %c2 : index
      %jj2 = arith.addi %j, %off20 : index
      %e20 = arith.muli %jj2, %stride : index
      %e21 = arith.addi %base, %e20 : index
      %e2 = arith.muli %e21, %i2 : index
      %e2i = arith.addi %e2, %i1 : index
      %xr2 = memref.load %Src[%e2] : memref<?xf32, strided<[1]>, 1>
      %xi2 = memref.load %Src[%e2i] : memref<?xf32, strided<[1]>, 1>
      %tq2 = arith.muli %qs, %c2 : index
      %tq22 = arith.muli %tq2, %i2 : index
      %tq22i = arith.addi %tq22, %i1 : index
      %wr2 = memref.load %Tw[%tq22] : memref<?xf32, strided<[1]>, 1>
      %wi20 = memref.load %Tw[%tq22i] : memref<?xf32, strided<[1]>, 1>
      %wi2n = arith.negf %wi20 : f32
      %wi2 = arith.select %is_inv, %wi2n, %wi20 : f32
      %a2r0 = arith.mulf %xr2, %wr2 : f32
      %a2r1 = arith.mulf %xi2, %wi2 : f32
      %tr2 = arith.subf %a2r0, %a2r1 : f32
      %a2i0 = arith.mulf %xr2, %wi2 : f32
      %ti2 = math.fma %xi2, %wr2, %a2i0 : f32
      %c3 = arith.constant 3 : index
      %off30 = arith.muli %quarter, %c3 : index
      %jj3 = arith.addi %j, %off30 : index
      %e30 = arith.muli %jj3, %stride : index
      %e31 = arith.addi %base, %e30 : index
      %e3 = arith.muli %e31, %i2 : index
      %e3i = arith.addi %e3, %i1 : index
      %xr3 = memref.load %Src[%e3] : memref<?xf32, strided<[1]>, 1>
      %xi3 = memref.load %Src[%e3i] : memref<?xf32, strided<[1]>, 1>
      %tq3 = arith.muli %qs, %c3 : index
      %tq32 = arith.muli %tq3, %i2 : index
      %tq32i = arith.addi %tq32, %i1 : index
      %wr3 = memref.load %Tw[%tq32] : memref<?xf32, strided<[1]>, 1>
      %wi30 = memref.load %Tw[%tq32i] : memref<?xf32, strided<[1]>, 1>
      %wi3n = arith.negf %wi30 : f32
      %wi3 = arith.select %is_inv, %wi3n, %wi30 : f32
      %a3r0 = arith.mulf %xr3, %wr3 : f32
      %a3r1 = arith.mulf %xi3, %wi3 : f32
      %tr3 = arith.subf %a3r0, %a3r1 : f32
      %a3i0 = arith.mulf %xr3, %wi3 : f32
      %ti3 = math.fma %xi3, %wr3, %a3i0 : f32
      // 4-point DFT: s02 = t0 + t2, d02 = t0 - t2, s13 = t1 + t3, d13 = t1 - t3;
      // y0 = s02 + s13, y2 = s02 - s13, y1 = d02 - i d13, y3 = d02 + i d13 (forward;
      // the inverse swaps y1 and y3)
      %s02r = arith.addf %xr0, %tr2 : f32
      %s02i = arith.addf %xi0, %ti2 : f32
      %d02r = arith.subf %xr0, %tr2 : f32
      %d02i = arith.subf %xi0, %ti2 : f32
      %s13r = arith.addf %tr1, %tr3 : f32
      %s13i = arith.addf %ti1, %ti3 : f32
      %d13r = arith.subf %tr1, %tr3 : f32
      %d13i = arith.subf %ti1, %ti3 : f32
      %y0r = arith.addf %s02r, %s13r : f32
      %y0i = arith.addf %s02i, %s13i : f32
      %y2r = arith.subf %s02r, %s13r : f32
      %y2i = arith.subf %s02i, %s13i : f32
      // - i d13 = (d13i, -d13r)
      %ma_r = arith.addf %d02r, %d13i : f32
      %ma_i = arith.subf %d02i, %d13r : f32
      %mb_r = arith.subf %d02r, %d13i : f32
      %mb_i = arith.addf %d02i, %d13r : f32
      %y1r = arith.select %is_inv, %mb_r, %ma_r : f32
      %y1i = arith.select %is_inv, %mb_i, %ma_i : f32
      %y3r = arith.select %is_inv, %ma_r, %mb_r : f32
      %y3i = arith.select %is_inv, %ma_i, %mb_i : f32
      %jd0 = arith.divui %j, %ns : index
      %jd1 = arith.muli %jd0, %ns4 : index
      %jd = arith.addi %jd1, %q : index
      %o00 = arith.muli %ns, %c0 : index
      %o01 = arith.addi %jd, %o00 : index
      %o02 = arith.muli %o01, %stride : index
      %o03 = arith.addi %base, %o02 : index
      %o0 = arith.muli %o03, %i2 : index
      %o0i = arith.addi %o0, %i1 : index
      memref.store %y0r, %Dst[%o0] : memref<?xf32, strided<[1]>, 1>
      memref.store %y0i, %Dst[%o0i] : memref<?xf32, strided<[1]>, 1>
      %o10 = arith.muli %ns, %c1 : index
      %o11 = arith.addi %jd, %o10 : index
      %o12 = arith.muli %o11, %stride : index
      %o13 = arith.addi %base, %o12 : index
      %o1 = arith.muli %o13, %i2 : index
      %o1i = arith.addi %o1, %i1 : index
      memref.store %y1r, %Dst[%o1] : memref<?xf32, strided<[1]>, 1>
      memref.store %y1i, %Dst[%o1i] : memref<?xf32, strided<[1]>, 1>
      %o20 = arith.muli %ns, %c2 : index
      %o21 = arith.addi %jd, %o20 : index
      %o22 = arith.muli %o21, %stride : index
      %o23 = arith.addi %base, %o22 : index
      %o2 = arith.muli %o23, %i2 : index
      %o2i = arith.addi %o2, %i1 : index
      memref.store %y2r, %Dst[%o2] : memref<?xf32, strided<[1]>, 1>
      memref.store %y2i, %Dst[%o2i] : memref<?xf32, strided<[1]>, 1>
      %o30 = arith.muli %ns, %c3 : index
      %o31 = arith.addi %jd, %o30 : index
      %o32 = arith.muli %o31, %stride : index
      %o33 = arith.addi %base, %o32 : index
      %o3 = arith.muli %o33, %i2 : index
      %o3i = arith.addi %o3, %i1 : index
      memref.store %y3r, %Dst[%o3] : memref<?xf32, strided<[1]>, 1>
      memref.store %y3i, %Dst[%o3i] : memref<?xf32, strided<[1]>, 1>
    }
    gpu.return
  }

  // The direct transform for any n: dst[k] = sum over j of src[j] tw[(j k) mod
  // n] (conjugated when inverse). Launch an n x batch grid; src and dst must
  // not overlap.
  gpu.func @agx_dft(%src0: memref<1xf32, 1>, %dst0: memref<1xf32, 1>,
                    %tw0: memref<1xf32, 1>, %n_: i32, %batch_: i32,
                    %stride_: i32, %dist_: i32, %inverse_: i32) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %i2 = arith.constant 2 : index
    %f0 = arith.constant 0.0 : f32
    %big = arith.constant 4611686018427387904 : index
    %n = arith.index_cast %n_ : i32 to index
    %batch = arith.index_cast %batch_ : i32 to index
    %stride = arith.index_cast %stride_ : i32 to index
    %dist = arith.index_cast %dist_ : i32 to index
    %inverse = arith.index_cast %inverse_ : i32 to index
    %Src = memref.reinterpret_cast %src0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %Dst = memref.reinterpret_cast %dst0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %Tw = memref.reinterpret_cast %tw0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %k = gpu.global_id x
    %b = gpu.global_id y
    %in_k = arith.cmpi ult, %k, %n : index
    %in_b = arith.cmpi ult, %b, %batch : index
    %live = arith.andi %in_k, %in_b : i1
    scf.if %live {
      %base = arith.muli %b, %dist : index
      %is_inv = arith.cmpi ne, %inverse, %i0 : index
      %sum:3 = scf.for %j = %i0 to %n step %i1 iter_args(%sr = %f0, %si = %f0, %t = %i0) -> (f32, f32, index) {
        %ij0 = arith.muli %j, %stride : index
        %ij1 = arith.addi %base, %ij0 : index
        %ij = arith.muli %ij1, %i2 : index
        %ij_im = arith.addi %ij, %i1 : index
        %xr = memref.load %Src[%ij] : memref<?xf32, strided<[1]>, 1>
        %xi = memref.load %Src[%ij_im] : memref<?xf32, strided<[1]>, 1>
        %t2 = arith.muli %t, %i2 : index
        %t2i = arith.addi %t2, %i1 : index
        %wr = memref.load %Tw[%t2] : memref<?xf32, strided<[1]>, 1>
        %wi0 = memref.load %Tw[%t2i] : memref<?xf32, strided<[1]>, 1>
        %wi_neg = arith.negf %wi0 : f32
        %wi = arith.select %is_inv, %wi_neg, %wi0 : f32
        // (xr + i xi)(wr + i wi)
        %p0 = math.fma %xr, %wr, %sr : f32
        %ni = arith.negf %xi : f32
        %sr1 = math.fma %ni, %wi, %p0 : f32
        %q0 = math.fma %xr, %wi, %si : f32
        %si1 = math.fma %xi, %wr, %q0 : f32
        // next twiddle index (j + 1) k mod n
        %t_n0 = arith.addi %t, %k : index
        %wrap = arith.cmpi uge, %t_n0, %n : index
        %t_n1 = arith.subi %t_n0, %n : index
        %t_n = arith.select %wrap, %t_n1, %t_n0 : index
        scf.yield %sr1, %si1, %t_n : f32, f32, index
      }
      %ok0 = arith.muli %k, %stride : index
      %ok1 = arith.addi %base, %ok0 : index
      %ok = arith.muli %ok1, %i2 : index
      %ok_im = arith.addi %ok, %i1 : index
      memref.store %sum#0, %Dst[%ok] : memref<?xf32, strided<[1]>, 1>
      memref.store %sum#1, %Dst[%ok_im] : memref<?xf32, strided<[1]>, 1>
    }
    gpu.return
  }

  // Real to complex input: dst (complex, n per sequence, contiguous) =
  // src (real, n per sequence, idist apart) with zero imaginary parts.
  // Launch an n x batch grid.
  gpu.func @agx_fft_real_to_complex(%src0: memref<1xf32, 1>, %dst0: memref<1xf32, 1>,
                                    %n_: i32, %batch_: i32, %idist_: i32) kernel {
    %i1 = arith.constant 1 : index
    %i2 = arith.constant 2 : index
    %f0 = arith.constant 0.0 : f32
    %big = arith.constant 4611686018427387904 : index
    %n = arith.index_cast %n_ : i32 to index
    %batch = arith.index_cast %batch_ : i32 to index
    %idist = arith.index_cast %idist_ : i32 to index
    %Src = memref.reinterpret_cast %src0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %Dst = memref.reinterpret_cast %dst0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %i = gpu.global_id x
    %b = gpu.global_id y
    %in_i = arith.cmpi ult, %i, %n : index
    %in_b = arith.cmpi ult, %b, %batch : index
    %live = arith.andi %in_i, %in_b : i1
    scf.if %live {
      %s0 = arith.muli %b, %idist : index
      %s = arith.addi %s0, %i : index
      %x = memref.load %Src[%s] : memref<?xf32, strided<[1]>, 1>
      %d0 = arith.muli %b, %n : index
      %d1 = arith.addi %d0, %i : index
      %d = arith.muli %d1, %i2 : index
      %d_im = arith.addi %d, %i1 : index
      memref.store %x, %Dst[%d] : memref<?xf32, strided<[1]>, 1>
      memref.store %f0, %Dst[%d_im] : memref<?xf32, strided<[1]>, 1>
    }
    gpu.return
  }

  // Copies the first m complex elements of each of `batch` sequences:
  // dst[b odist + i ostride] = src[b idist + i istride] (complex elements).
  // Launch an m x batch grid. Used for the n / 2 + 1 outputs of real to
  // complex transforms and to finish in-place transforms exactly (the
  // elements between strided ones are left alone).
  gpu.func @agx_fft_copy(%src0: memref<1xf32, 1>, %dst0: memref<1xf32, 1>,
                         %m_: i32, %batch_: i32, %istride_: i32, %idist_: i32,
                         %ostride_: i32, %odist_: i32) kernel {
    %i1 = arith.constant 1 : index
    %i2 = arith.constant 2 : index
    %big = arith.constant 4611686018427387904 : index
    %m = arith.index_cast %m_ : i32 to index
    %batch = arith.index_cast %batch_ : i32 to index
    %istride = arith.index_cast %istride_ : i32 to index
    %idist = arith.index_cast %idist_ : i32 to index
    %ostride = arith.index_cast %ostride_ : i32 to index
    %odist = arith.index_cast %odist_ : i32 to index
    %Src = memref.reinterpret_cast %src0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %Dst = memref.reinterpret_cast %dst0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %i = gpu.global_id x
    %b = gpu.global_id y
    %in_i = arith.cmpi ult, %i, %m : index
    %in_b = arith.cmpi ult, %b, %batch : index
    %live = arith.andi %in_i, %in_b : i1
    scf.if %live {
      %s0 = arith.muli %b, %idist : index
      %si = arith.muli %i, %istride : index
      %s1 = arith.addi %s0, %si : index
      %s = arith.muli %s1, %i2 : index
      %s_im = arith.addi %s, %i1 : index
      %d0 = arith.muli %b, %odist : index
      %di = arith.muli %i, %ostride : index
      %d1 = arith.addi %d0, %di : index
      %d = arith.muli %d1, %i2 : index
      %d_im = arith.addi %d, %i1 : index
      %xr = memref.load %Src[%s] : memref<?xf32, strided<[1]>, 1>
      %xi = memref.load %Src[%s_im] : memref<?xf32, strided<[1]>, 1>
      memref.store %xr, %Dst[%d] : memref<?xf32, strided<[1]>, 1>
      memref.store %xi, %Dst[%d_im] : memref<?xf32, strided<[1]>, 1>
    }
    gpu.return
  }

  // Complex to real input: dst (complex, n per sequence, contiguous) is the
  // full Hermitian spectrum of src (n / 2 + 1 complex elements per sequence,
  // idist apart): dst[k] = src[k] for k <= n / 2, conj(src[n - k]) above.
  // Launch an n x batch grid.
  gpu.func @agx_fft_hermitian(%src0: memref<1xf32, 1>, %dst0: memref<1xf32, 1>,
                              %n_: i32, %batch_: i32, %idist_: i32) kernel {
    %i1 = arith.constant 1 : index
    %i2 = arith.constant 2 : index
    %big = arith.constant 4611686018427387904 : index
    %n = arith.index_cast %n_ : i32 to index
    %batch = arith.index_cast %batch_ : i32 to index
    %idist = arith.index_cast %idist_ : i32 to index
    %Src = memref.reinterpret_cast %src0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %Dst = memref.reinterpret_cast %dst0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %k = gpu.global_id x
    %b = gpu.global_id y
    %in_k = arith.cmpi ult, %k, %n : index
    %in_b = arith.cmpi ult, %b, %batch : index
    %live = arith.andi %in_k, %in_b : i1
    scf.if %live {
      %h = arith.divui %n, %i2 : index
      %low = arith.cmpi ule, %k, %h : index
      %mirror = arith.subi %n, %k : index
      %j = arith.select %low, %k, %mirror : index
      %s0 = arith.muli %b, %idist : index
      %s1 = arith.addi %s0, %j : index
      %s = arith.muli %s1, %i2 : index
      %s_im = arith.addi %s, %i1 : index
      %xr = memref.load %Src[%s] : memref<?xf32, strided<[1]>, 1>
      %xi0 = memref.load %Src[%s_im] : memref<?xf32, strided<[1]>, 1>
      %xin = arith.negf %xi0 : f32
      %xi = arith.select %low, %xi0, %xin : f32
      %d0 = arith.muli %b, %n : index
      %d1 = arith.addi %d0, %k : index
      %d = arith.muli %d1, %i2 : index
      %d_im = arith.addi %d, %i1 : index
      memref.store %xr, %Dst[%d] : memref<?xf32, strided<[1]>, 1>
      memref.store %xi, %Dst[%d_im] : memref<?xf32, strided<[1]>, 1>
    }
    gpu.return
  }

  // The real parts of src (complex, n per sequence, contiguous) into dst
  // (real, odist apart). Launch an n x batch grid.
  gpu.func @agx_fft_real_part(%src0: memref<1xf32, 1>, %dst0: memref<1xf32, 1>,
                              %n_: i32, %batch_: i32, %odist_: i32) kernel {
    %i2 = arith.constant 2 : index
    %big = arith.constant 4611686018427387904 : index
    %n = arith.index_cast %n_ : i32 to index
    %batch = arith.index_cast %batch_ : i32 to index
    %odist = arith.index_cast %odist_ : i32 to index
    %Src = memref.reinterpret_cast %src0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %Dst = memref.reinterpret_cast %dst0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %i = gpu.global_id x
    %b = gpu.global_id y
    %in_i = arith.cmpi ult, %i, %n : index
    %in_b = arith.cmpi ult, %b, %batch : index
    %live = arith.andi %in_i, %in_b : i1
    scf.if %live {
      %s0 = arith.muli %b, %n : index
      %s1 = arith.addi %s0, %i : index
      %s = arith.muli %s1, %i2 : index
      %x = memref.load %Src[%s] : memref<?xf32, strided<[1]>, 1>
      %d0 = arith.muli %b, %odist : index
      %d = arith.addi %d0, %i : index
      memref.store %x, %Dst[%d] : memref<?xf32, strided<[1]>, 1>
    }
    gpu.return
  }
}
