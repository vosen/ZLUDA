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
