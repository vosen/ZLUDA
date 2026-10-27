// Pointwise, softmax, pooling and batch normalization kernels of zluda_dnn_agx
// (cuDNN), in f32 for every AGX generation like sgemm.mlir (f16 and bf16
// tensors go through f32 copies, agx_dnn_to_f32 / agx_dnn_from_f32). Tensors are 4-D
// N x C x H x W with arbitrary element strides (0 broadcasts); a thread's
// linear index runs over N C H W in that order. Outputs follow cuDNN's
// blending, y = alpha r + beta y, and y is not read when beta is 0.
// Activations, softmax and erf use the hardware's exp and log
// (fastmath<afn>, a few units in the last place, as cuDNN's fast forms);
// the pointwise math modes (exp, log, pow, sin, cos, tan) are precise.

gpu.module @agx_dnn {
  func.func private @agx_dnn_coord(%i: index, %C: index, %H: index, %W: index)
      -> (index, index, index, index) {
    %w = arith.remui %i, %W : index
    %t0 = arith.divui %i, %W : index
    %h = arith.remui %t0, %H : index
    %t1 = arith.divui %t0, %H : index
    %c = arith.remui %t1, %C : index
    %n = arith.divui %t1, %C : index
    return %n, %c, %h, %w : index, index, index, index
  }

  func.func private @agx_dnn_dot4(%n: index, %c: index, %h: index, %w: index,
      %s0: index, %s1: index, %s2: index, %s3: index) -> index {
    %a = arith.muli %n, %s0 : index
    %b = arith.muli %c, %s1 : index
    %d = arith.muli %h, %s2 : index
    %e = arith.muli %w, %s3 : index
    %ab = arith.addi %a, %b : index
    %de = arith.addi %d, %e : index
    %o = arith.addi %ab, %de : index
    return %o : index
  }

  func.func private @agx_dnn_off(%i: index, %C: index, %H: index, %W: index,
      %s0: index, %s1: index, %s2: index, %s3: index) -> index {
    %n, %c, %h, %w = func.call @agx_dnn_coord(%i, %C, %H, %W)
        : (index, index, index, index) -> (index, index, index, index)
    %o = func.call @agx_dnn_dot4(%n, %c, %h, %w, %s0, %s1, %s2, %s3)
        : (index, index, index, index, index, index, index, index) -> index
    return %o : index
  }

  // alpha r + beta old, without reading old when beta is 0
  func.func private @agx_dnn_blend(%y0: memref<1xf32, 1>, %o: index,
      %r: f32, %alpha: f32, %beta: f32) {
    %big = arith.constant 4611686018427387904 : index
    %y = memref.reinterpret_cast %y0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %f0 = arith.constant 0.0 : f32
    %ar = arith.mulf %r, %alpha : f32
    %zero = arith.cmpf oeq, %beta, %f0 : f32
    %v = scf.if %zero -> f32 {
      scf.yield %ar : f32
    } else {
      %old = memref.load %y[%o] : memref<?xf32, strided<[1]>, 1>
      %s = math.fma %old, %beta, %ar : f32
      scf.yield %s : f32
    }
    memref.store %v, %y[%o] : memref<?xf32, strided<[1]>, 1>
    return
  }

  // 1 / (1 + e^-x)
  func.func private @agx_dnn_sigmoid(%x: f32) -> f32 {
    %f1 = arith.constant 1.0 : f32
    %nlog2e = arith.constant -1.44269504 : f32
    %a = arith.mulf %x, %nlog2e : f32
    %e = math.exp2 %a fastmath<afn> : f32
    %d = arith.addf %e, %f1 : f32
    %r = arith.divf %f1, %d : f32
    return %r : f32
  }

  // e^x - 1, with a series near 0
  func.func private @agx_dnn_expm1(%x: f32) -> f32 {
    %f1 = arith.constant 1.0 : f32
    %log2e = arith.constant 1.44269504 : f32
    %small = arith.constant 0.03125 : f32
    %c2 = arith.constant 0.5 : f32
    %c3 = arith.constant 0.166666672 : f32
    %c4 = arith.constant 0.0416666679 : f32
    %ax = math.absf %x : f32
    %near = arith.cmpf olt, %ax, %small : f32
    %a = arith.mulf %x, %log2e : f32
    %e = math.exp2 %a fastmath<afn> : f32
    %far = arith.subf %e, %f1 : f32
    // x + x^2/2 + x^3/6 + x^4/24
    %p0 = math.fma %x, %c4, %c3 : f32
    %p1 = math.fma %x, %p0, %c2 : f32
    %x2 = arith.mulf %x, %x : f32
    %ser = math.fma %x2, %p1, %x : f32
    %r = arith.select %near, %ser, %far : f32
    return %r : f32
  }

  // tanh x = sign(x) (1 - e) / (1 + e), e = e^-2|x|; a series near 0
  func.func private @agx_dnn_tanh(%x: f32) -> f32 {
    %f0 = arith.constant 0.0 : f32
    %f1 = arith.constant 1.0 : f32
    %m2 = arith.constant -2.0 : f32
    %small = arith.constant 0.0625 : f32
    %c3 = arith.constant -0.333333343 : f32
    %c5 = arith.constant 0.13333334 : f32
    %c7 = arith.constant -0.0539682545 : f32
    %ax = math.absf %x : f32
    %t = arith.mulf %ax, %m2 : f32
    %em1 = func.call @agx_dnn_expm1(%t) : (f32) -> f32
    // (1 - e) / (1 + e) = -expm1 / (2 + expm1)
    %f2 = arith.constant 2.0 : f32
    %num = arith.negf %em1 : f32
    %den = arith.addf %em1, %f2 : f32
    %q = arith.divf %num, %den : f32
    %neg = arith.cmpf olt, %x, %f0 : f32
    %nq = arith.negf %q : f32
    %far = arith.select %neg, %nq, %q : f32
    %x2 = arith.mulf %x, %x : f32
    %p0 = math.fma %x2, %c7, %c5 : f32
    %p1 = math.fma %x2, %p0, %c3 : f32
    %x3 = arith.mulf %x2, %x : f32
    %ser = math.fma %x3, %p1, %x : f32
    %near = arith.cmpf olt, %ax, %small : f32
    %r = arith.select %near, %ser, %far : f32
    return %r : f32
  }

  // cuDNN activation modes: sigmoid 0, relu 1, tanh 2, clipped relu 3 (ceiling
  // coef), elu 4 (alpha coef), identity 5, swish 6 (beta coef)
  func.func private @agx_dnn_act(%x: f32, %mode: index, %coef: f32) -> f32 {
    %f0 = arith.constant 0.0 : f32
    %c0 = arith.constant 0 : index
    %c1 = arith.constant 1 : index
    %c2 = arith.constant 2 : index
    %c3 = arith.constant 3 : index
    %c4 = arith.constant 4 : index
    %c6 = arith.constant 6 : index
    %sig = func.call @agx_dnn_sigmoid(%x) : (f32) -> f32
    %th = func.call @agx_dnn_tanh(%x) : (f32) -> f32
    %relu = arith.maximumf %x, %f0 : f32
    %clip = arith.minimumf %relu, %coef : f32
    %em1 = func.call @agx_dnn_expm1(%x) : (f32) -> f32
    %eneg = arith.mulf %em1, %coef : f32
    %pos = arith.cmpf ogt, %x, %f0 : f32
    %elu = arith.select %pos, %x, %eneg : f32
    %bx = arith.mulf %x, %coef : f32
    %sb = func.call @agx_dnn_sigmoid(%bx) : (f32) -> f32
    %swish = arith.mulf %x, %sb : f32
    %is0 = arith.cmpi eq, %mode, %c0 : index
    %is1 = arith.cmpi eq, %mode, %c1 : index
    %is2 = arith.cmpi eq, %mode, %c2 : index
    %is3 = arith.cmpi eq, %mode, %c3 : index
    %is4 = arith.cmpi eq, %mode, %c4 : index
    %is6 = arith.cmpi eq, %mode, %c6 : index
    %r6 = arith.select %is6, %swish, %x : f32
    %r4 = arith.select %is4, %elu, %r6 : f32
    %r3 = arith.select %is3, %clip, %r4 : f32
    %r2 = arith.select %is2, %th, %r3 : f32
    %r1 = arith.select %is1, %relu, %r2 : f32
    %r0 = arith.select %is0, %sig, %r1 : f32
    return %r0 : f32
  }

  // dx for the forward input x, output y and gradient dy
  func.func private @agx_dnn_act_grad(%x: f32, %y: f32, %dy: f32, %mode: index, %coef: f32) -> f32 {
    %f0 = arith.constant 0.0 : f32
    %f1 = arith.constant 1.0 : f32
    %c0 = arith.constant 0 : index
    %c1 = arith.constant 1 : index
    %c2 = arith.constant 2 : index
    %c3 = arith.constant 3 : index
    %c4 = arith.constant 4 : index
    %c6 = arith.constant 6 : index
    %omy = arith.subf %f1, %y : f32
    %yomy = arith.mulf %y, %omy : f32
    %sig = arith.mulf %dy, %yomy : f32
    %pos = arith.cmpf ogt, %x, %f0 : f32
    %relu = arith.select %pos, %dy, %f0 : f32
    %y2 = arith.mulf %y, %y : f32
    %omy2 = arith.subf %f1, %y2 : f32
    %th = arith.mulf %dy, %omy2 : f32
    %below = arith.cmpf olt, %x, %coef : f32
    %inside = arith.andi %pos, %below : i1
    %clip = arith.select %inside, %dy, %f0 : f32
    %yc = arith.addf %y, %coef : f32
    %dyc = arith.mulf %dy, %yc : f32
    %elu = arith.select %pos, %dy, %dyc : f32
    %bx = arith.mulf %x, %coef : f32
    %s = func.call @agx_dnn_sigmoid(%bx) : (f32) -> f32
    %oms = arith.subf %f1, %s : f32
    %bxs = arith.mulf %bx, %s : f32
    %d0 = math.fma %bxs, %oms, %s : f32
    %swish = arith.mulf %dy, %d0 : f32
    %is0 = arith.cmpi eq, %mode, %c0 : index
    %is1 = arith.cmpi eq, %mode, %c1 : index
    %is2 = arith.cmpi eq, %mode, %c2 : index
    %is3 = arith.cmpi eq, %mode, %c3 : index
    %is4 = arith.cmpi eq, %mode, %c4 : index
    %is6 = arith.cmpi eq, %mode, %c6 : index
    %r6 = arith.select %is6, %swish, %dy : f32
    %r4 = arith.select %is4, %elu, %r6 : f32
    %r3 = arith.select %is3, %clip, %r4 : f32
    %r2 = arith.select %is2, %th, %r3 : f32
    %r1 = arith.select %is1, %relu, %r2 : f32
    %r0 = arith.select %is0, %sig, %r1 : f32
    return %r0 : f32
  }

  // y = alpha act(x) + beta y, one thread per element
  gpu.func @agx_dnn_act_fwd(%x0: memref<1xf32, 1>, %y0: memref<1xf32, 1>,
      %total: index, %C: index, %H: index, %W: index,
      %xs0: index, %xs1: index, %xs2: index, %xs3: index,
      %ys0: index, %ys1: index, %ys2: index, %ys3: index,
      %mode: index, %coef: f32, %alpha: f32, %beta: f32) kernel {
    %big_x = arith.constant 4611686018427387904 : index
    %x = memref.reinterpret_cast %x0 to offset: [0], sizes: [%big_x], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %big_y = arith.constant 4611686018427387904 : index
    %y = memref.reinterpret_cast %y0 to offset: [0], sizes: [%big_y], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %i = gpu.global_id x
    %live = arith.cmpi ult, %i, %total : index
    scf.if %live {
      %xo = func.call @agx_dnn_off(%i, %C, %H, %W, %xs0, %xs1, %xs2, %xs3)
          : (index, index, index, index, index, index, index, index) -> index
      %yo = func.call @agx_dnn_off(%i, %C, %H, %W, %ys0, %ys1, %ys2, %ys3)
          : (index, index, index, index, index, index, index, index) -> index
      %v = memref.load %x[%xo] : memref<?xf32, strided<[1]>, 1>
      %r = func.call @agx_dnn_act(%v, %mode, %coef) : (f32, index, f32) -> f32
      func.call @agx_dnn_blend(%y0, %yo, %r, %alpha, %beta) : (memref<1xf32, 1>, index, f32, f32, f32) -> ()
    }
    gpu.return
  }

  // dx = alpha act'(x, y) dy + beta dx, one thread per element
  gpu.func @agx_dnn_act_bwd(%y0: memref<1xf32, 1>, %dy0: memref<1xf32, 1>,
      %x0: memref<1xf32, 1>, %dx0: memref<1xf32, 1>,
      %total: index, %C: index, %H: index, %W: index,
      %ys0: index, %ys1: index, %ys2: index, %ys3: index,
      %dys0: index, %dys1: index, %dys2: index, %dys3: index,
      %xs0: index, %xs1: index, %xs2: index, %xs3: index,
      %dxs0: index, %dxs1: index, %dxs2: index, %dxs3: index,
      %mode: index, %coef: f32, %alpha: f32, %beta: f32) kernel {
    %big_y = arith.constant 4611686018427387904 : index
    %y = memref.reinterpret_cast %y0 to offset: [0], sizes: [%big_y], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %big_dy = arith.constant 4611686018427387904 : index
    %dy = memref.reinterpret_cast %dy0 to offset: [0], sizes: [%big_dy], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %big_x = arith.constant 4611686018427387904 : index
    %x = memref.reinterpret_cast %x0 to offset: [0], sizes: [%big_x], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %big_dx = arith.constant 4611686018427387904 : index
    %dx = memref.reinterpret_cast %dx0 to offset: [0], sizes: [%big_dx], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %i = gpu.global_id x
    %live = arith.cmpi ult, %i, %total : index
    scf.if %live {
      %n, %c, %h, %w = func.call @agx_dnn_coord(%i, %C, %H, %W)
          : (index, index, index, index) -> (index, index, index, index)
      %yo = func.call @agx_dnn_dot4(%n, %c, %h, %w, %ys0, %ys1, %ys2, %ys3)
          : (index, index, index, index, index, index, index, index) -> index
      %dyo = func.call @agx_dnn_dot4(%n, %c, %h, %w, %dys0, %dys1, %dys2, %dys3)
          : (index, index, index, index, index, index, index, index) -> index
      %xo = func.call @agx_dnn_dot4(%n, %c, %h, %w, %xs0, %xs1, %xs2, %xs3)
          : (index, index, index, index, index, index, index, index) -> index
      %dxo = func.call @agx_dnn_dot4(%n, %c, %h, %w, %dxs0, %dxs1, %dxs2, %dxs3)
          : (index, index, index, index, index, index, index, index) -> index
      %vy = memref.load %y[%yo] : memref<?xf32, strided<[1]>, 1>
      %vdy = memref.load %dy[%dyo] : memref<?xf32, strided<[1]>, 1>
      %vx = memref.load %x[%xo] : memref<?xf32, strided<[1]>, 1>
      %r = func.call @agx_dnn_act_grad(%vx, %vy, %vdy, %mode, %coef) : (f32, f32, f32, index, f32) -> f32
      func.call @agx_dnn_blend(%dx0, %dxo, %r, %alpha, %beta) : (memref<1xf32, 1>, index, f32, f32, f32) -> ()
    }
    gpu.return
  }

  // c = alpha a + beta c (a broadcast through 0 strides): add 0; y = value:
  // set 1 (a unused); y = value y: scale 2 (a unused)
  gpu.func @agx_dnn_pointwise(%a0: memref<1xf32, 1>, %c0: memref<1xf32, 1>,
      %total: index, %C: index, %H: index, %W: index,
      %as0: index, %as1: index, %as2: index, %as3: index,
      %cs0: index, %cs1: index, %cs2: index, %cs3: index,
      %op: index, %alpha: f32, %beta: f32) kernel {
    %big_a = arith.constant 4611686018427387904 : index
    %a = memref.reinterpret_cast %a0 to offset: [0], sizes: [%big_a], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %big_c = arith.constant 4611686018427387904 : index
    %c = memref.reinterpret_cast %c0 to offset: [0], sizes: [%big_c], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %k0 = arith.constant 0 : index
    %k1 = arith.constant 1 : index
    %i = gpu.global_id x
    %live = arith.cmpi ult, %i, %total : index
    scf.if %live {
      %n, %ch, %h, %w = func.call @agx_dnn_coord(%i, %C, %H, %W)
          : (index, index, index, index) -> (index, index, index, index)
      %co = func.call @agx_dnn_dot4(%n, %ch, %h, %w, %cs0, %cs1, %cs2, %cs3)
          : (index, index, index, index, index, index, index, index) -> index
      %is_add = arith.cmpi eq, %op, %k0 : index
      %is_set = arith.cmpi eq, %op, %k1 : index
      scf.if %is_add {
        %ao = func.call @agx_dnn_dot4(%n, %ch, %h, %w, %as0, %as1, %as2, %as3)
            : (index, index, index, index, index, index, index, index) -> index
        %v = memref.load %a[%ao] : memref<?xf32, strided<[1]>, 1>
        func.call @agx_dnn_blend(%c0, %co, %v, %alpha, %beta) : (memref<1xf32, 1>, index, f32, f32, f32) -> ()
      } else {
        scf.if %is_set {
          memref.store %alpha, %c[%co] : memref<?xf32, strided<[1]>, 1>
        } else {
          %old = memref.load %c[%co] : memref<?xf32, strided<[1]>, 1>
          %v = arith.mulf %old, %alpha : f32
          memref.store %v, %c[%co] : memref<?xf32, strided<[1]>, 1>
        }
      }
    }
    gpu.return
  }

  // The linear (N C H W) index of element k of softmax row o: over C for each
  // (n, h, w) in channel mode, over C H W for each n in instance mode.
  func.func private @agx_dnn_sm_index(%o: index, %k: index, %C: index, %HW: index, %instance: index) -> index {
    %k0 = arith.constant 0 : index
    %inst = arith.cmpi ne, %instance, %k0 : index
    %chw = arith.muli %C, %HW : index
    %ib = arith.muli %o, %chw : index
    %ii = arith.addi %ib, %k : index
    %n = arith.divui %o, %HW : index
    %hw = arith.remui %o, %HW : index
    %nc = arith.muli %n, %C : index
    %nck = arith.addi %nc, %k : index
    %cb = arith.muli %nck, %HW : index
    %ci = arith.addi %cb, %hw : index
    %r = arith.select %inst, %ii, %ci : index
    return %r : index
  }

  // Softmax forward over rows rows of K elements; algo fast 0 (no maximum
  // subtracted), accurate 1, log 2. One thread per row.
  gpu.func @agx_dnn_softmax_fwd(%x0: memref<1xf32, 1>, %y0: memref<1xf32, 1>,
      %rows: index, %K: index, %C: index, %H: index, %W: index,
      %xs0: index, %xs1: index, %xs2: index, %xs3: index,
      %ys0: index, %ys1: index, %ys2: index, %ys3: index,
      %instance: index, %algo: index, %alpha: f32, %beta: f32) kernel {
    %big_x = arith.constant 4611686018427387904 : index
    %x = memref.reinterpret_cast %x0 to offset: [0], sizes: [%big_x], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %big_y = arith.constant 4611686018427387904 : index
    %y = memref.reinterpret_cast %y0 to offset: [0], sizes: [%big_y], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %k0 = arith.constant 0 : index
    %k1 = arith.constant 1 : index
    %k2 = arith.constant 2 : index
    %f0 = arith.constant 0.0 : f32
    %ninf = arith.constant 0xFF800000 : f32
    %log2e = arith.constant 1.44269504 : f32
    %ln2 = arith.constant 0.693147182 : f32
    %HW = arith.muli %H, %W : index
    %o = gpu.global_id x
    %live = arith.cmpi ult, %o, %rows : index
    scf.if %live {
      %fast = arith.cmpi eq, %algo, %k0 : index
      %log = arith.cmpi eq, %algo, %k2 : index
      %m = scf.if %fast -> f32 {
        scf.yield %f0 : f32
      } else {
        %mx = scf.for %k = %k0 to %K step %k1 iter_args(%acc = %ninf) -> f32 {
          %li = func.call @agx_dnn_sm_index(%o, %k, %C, %HW, %instance) : (index, index, index, index, index) -> index
          %xo = func.call @agx_dnn_off(%li, %C, %H, %W, %xs0, %xs1, %xs2, %xs3)
              : (index, index, index, index, index, index, index, index) -> index
          %v = memref.load %x[%xo] : memref<?xf32, strided<[1]>, 1>
          %a = arith.maximumf %acc, %v : f32
          scf.yield %a : f32
        }
        scf.yield %mx : f32
      }
      %s = scf.for %k = %k0 to %K step %k1 iter_args(%acc = %f0) -> f32 {
        %li = func.call @agx_dnn_sm_index(%o, %k, %C, %HW, %instance) : (index, index, index, index, index) -> index
        %xo = func.call @agx_dnn_off(%li, %C, %H, %W, %xs0, %xs1, %xs2, %xs3)
            : (index, index, index, index, index, index, index, index) -> index
        %v = memref.load %x[%xo] : memref<?xf32, strided<[1]>, 1>
        %d = arith.subf %v, %m : f32
        %dl = arith.mulf %d, %log2e : f32
        %e = math.exp2 %dl fastmath<afn> : f32
        %a = arith.addf %acc, %e : f32
        scf.yield %a : f32
      }
      %l2 = math.log2 %s fastmath<afn> : f32
      %lns = arith.mulf %l2, %ln2 : f32
      scf.for %k = %k0 to %K step %k1 {
        %li = func.call @agx_dnn_sm_index(%o, %k, %C, %HW, %instance) : (index, index, index, index, index) -> index
        %xo = func.call @agx_dnn_off(%li, %C, %H, %W, %xs0, %xs1, %xs2, %xs3)
            : (index, index, index, index, index, index, index, index) -> index
        %yo = func.call @agx_dnn_off(%li, %C, %H, %W, %ys0, %ys1, %ys2, %ys3)
            : (index, index, index, index, index, index, index, index) -> index
        %v = memref.load %x[%xo] : memref<?xf32, strided<[1]>, 1>
        %d = arith.subf %v, %m : f32
        %dl = arith.mulf %d, %log2e : f32
        %e = math.exp2 %dl fastmath<afn> : f32
        %p = arith.divf %e, %s : f32
        %lp = arith.subf %d, %lns : f32
        %r = arith.select %log, %lp, %p : f32
        func.call @agx_dnn_blend(%y0, %yo, %r, %alpha, %beta) : (memref<1xf32, 1>, index, f32, f32, f32) -> ()
      }
    }
    gpu.return
  }

  // Softmax backward: dx = y (dy - sum dy y), or dy - e^y sum dy for log.
  gpu.func @agx_dnn_softmax_bwd(%y0: memref<1xf32, 1>, %dy0: memref<1xf32, 1>,
      %dx0: memref<1xf32, 1>, %rows: index, %K: index, %C: index, %H: index, %W: index,
      %ys0: index, %ys1: index, %ys2: index, %ys3: index,
      %dys0: index, %dys1: index, %dys2: index, %dys3: index,
      %dxs0: index, %dxs1: index, %dxs2: index, %dxs3: index,
      %instance: index, %algo: index, %alpha: f32, %beta: f32) kernel {
    %big_y = arith.constant 4611686018427387904 : index
    %y = memref.reinterpret_cast %y0 to offset: [0], sizes: [%big_y], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %big_dy = arith.constant 4611686018427387904 : index
    %dy = memref.reinterpret_cast %dy0 to offset: [0], sizes: [%big_dy], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %big_dx = arith.constant 4611686018427387904 : index
    %dx = memref.reinterpret_cast %dx0 to offset: [0], sizes: [%big_dx], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %k0 = arith.constant 0 : index
    %k1 = arith.constant 1 : index
    %k2 = arith.constant 2 : index
    %f0 = arith.constant 0.0 : f32
    %log2e = arith.constant 1.44269504 : f32
    %HW = arith.muli %H, %W : index
    %o = gpu.global_id x
    %live = arith.cmpi ult, %o, %rows : index
    scf.if %live {
      %log = arith.cmpi eq, %algo, %k2 : index
      %s = scf.for %k = %k0 to %K step %k1 iter_args(%acc = %f0) -> f32 {
        %li = func.call @agx_dnn_sm_index(%o, %k, %C, %HW, %instance) : (index, index, index, index, index) -> index
        %yo = func.call @agx_dnn_off(%li, %C, %H, %W, %ys0, %ys1, %ys2, %ys3)
            : (index, index, index, index, index, index, index, index) -> index
        %dyo = func.call @agx_dnn_off(%li, %C, %H, %W, %dys0, %dys1, %dys2, %dys3)
            : (index, index, index, index, index, index, index, index) -> index
        %vy = memref.load %y[%yo] : memref<?xf32, strided<[1]>, 1>
        %vdy = memref.load %dy[%dyo] : memref<?xf32, strided<[1]>, 1>
        %p = arith.mulf %vy, %vdy : f32
        %t = arith.select %log, %vdy, %p : f32
        %a = arith.addf %acc, %t : f32
        scf.yield %a : f32
      }
      scf.for %k = %k0 to %K step %k1 {
        %li = func.call @agx_dnn_sm_index(%o, %k, %C, %HW, %instance) : (index, index, index, index, index) -> index
        %yo = func.call @agx_dnn_off(%li, %C, %H, %W, %ys0, %ys1, %ys2, %ys3)
            : (index, index, index, index, index, index, index, index) -> index
        %dyo = func.call @agx_dnn_off(%li, %C, %H, %W, %dys0, %dys1, %dys2, %dys3)
            : (index, index, index, index, index, index, index, index) -> index
        %dxo = func.call @agx_dnn_off(%li, %C, %H, %W, %dxs0, %dxs1, %dxs2, %dxs3)
            : (index, index, index, index, index, index, index, index) -> index
        %vy = memref.load %y[%yo] : memref<?xf32, strided<[1]>, 1>
        %vdy = memref.load %dy[%dyo] : memref<?xf32, strided<[1]>, 1>
        %d = arith.subf %vdy, %s : f32
        %plain = arith.mulf %vy, %d : f32
        %yl = arith.mulf %vy, %log2e : f32
        %e = math.exp2 %yl fastmath<afn> : f32
        %es = arith.mulf %e, %s : f32
        %lg = arith.subf %vdy, %es : f32
        %r = arith.select %log, %lg, %plain : f32
        func.call @agx_dnn_blend(%dx0, %dxo, %r, %alpha, %beta) : (memref<1xf32, 1>, index, f32, f32, f32) -> ()
      }
    }
    gpu.return
  }

  // The first window position (r, s) holding the maximum of output (n, c, p,
  // q), or R when the window has no element inside x.
  func.func private @agx_dnn_pool_argmax(%x0: memref<1xf32, 1>,
      %n: index, %c: index, %p: index, %q: index, %H: index, %W: index,
      %R: index, %S: index, %ph: index, %pw: index, %sh: index, %sw: index,
      %xs0: index, %xs1: index, %xs2: index, %xs3: index) -> (index, index) {
    %big = arith.constant 4611686018427387904 : index
    %x = memref.reinterpret_cast %x0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %k0 = arith.constant 0 : index
    %k1 = arith.constant 1 : index
    %ninf = arith.constant 0xFF800000 : f32
    %h0a = arith.muli %p, %sh : index
    %h0 = arith.subi %h0a, %ph : index
    %w0a = arith.muli %q, %sw : index
    %w0 = arith.subi %w0a, %pw : index
    %res:4 = scf.for %r = %k0 to %R step %k1 iter_args(%best = %ninf, %br = %R, %bs = %k0, %any = %k0) -> (f32, index, index, index) {
      %h = arith.addi %h0, %r : index
      %hin0 = arith.cmpi sge, %h, %k0 : index
      %hin1 = arith.cmpi slt, %h, %H : index
      %hin = arith.andi %hin0, %hin1 : i1
      %inner:4 = scf.for %s = %k0 to %S step %k1 iter_args(%b2 = %best, %r2 = %br, %s2 = %bs, %a2 = %any) -> (f32, index, index, index) {
        %w = arith.addi %w0, %s : index
        %win0 = arith.cmpi sge, %w, %k0 : index
        %win1 = arith.cmpi slt, %w, %W : index
        %win = arith.andi %win0, %win1 : i1
        %in = arith.andi %hin, %win : i1
        %u:4 = scf.if %in -> (f32, index, index, index) {
          %o = func.call @agx_dnn_dot4(%n, %c, %h, %w, %xs0, %xs1, %xs2, %xs3)
              : (index, index, index, index, index, index, index, index) -> index
          %v = memref.load %x[%o] : memref<?xf32, strided<[1]>, 1>
          %gt = arith.cmpf ogt, %v, %b2 : f32
          %first = arith.cmpi eq, %a2, %k0 : index
          %take = arith.ori %gt, %first : i1
          %nb = arith.select %take, %v, %b2 : f32
          %nr = arith.select %take, %r, %r2 : index
          %ns = arith.select %take, %s, %s2 : index
          scf.yield %nb, %nr, %ns, %k1 : f32, index, index, index
        } else {
          scf.yield %b2, %r2, %s2, %a2 : f32, index, index, index
        }
        scf.yield %u#0, %u#1, %u#2, %u#3 : f32, index, index, index
      }
      scf.yield %inner#0, %inner#1, %inner#2, %inner#3 : f32, index, index, index
    }
    return %res#1, %res#2 : index, index
  }

  // The divisor of an average window: R S (include padding, mode 1) or the
  // elements inside x (exclude padding, mode 2).
  func.func private @agx_dnn_pool_count(%p: index, %q: index, %H: index, %W: index,
      %R: index, %S: index, %ph: index, %pw: index, %sh: index, %sw: index,
      %mode: index) -> f32 {
    %k0 = arith.constant 0 : index
    %k1 = arith.constant 1 : index
    %k2 = arith.constant 2 : index
    %h0a = arith.muli %p, %sh : index
    %h0 = arith.subi %h0a, %ph : index
    %w0a = arith.muli %q, %sw : index
    %w0 = arith.subi %w0a, %pw : index
    %h1 = arith.addi %h0, %R : index
    %w1 = arith.addi %w0, %S : index
    %hlo = arith.maxsi %h0, %k0 : index
    %hhi = arith.minsi %h1, %H : index
    %wlo = arith.maxsi %w0, %k0 : index
    %whi = arith.minsi %w1, %W : index
    %hn0 = arith.subi %hhi, %hlo : index
    %wn0 = arith.subi %whi, %wlo : index
    %hn = arith.maxsi %hn0, %k0 : index
    %wn = arith.maxsi %wn0, %k0 : index
    %inside = arith.muli %hn, %wn : index
    %all = arith.muli %R, %S : index
    %ex = arith.cmpi eq, %mode, %k2 : index
    %cnt = arith.select %ex, %inside, %all : index
    %one = arith.maxsi %cnt, %k1 : index
    %ci = arith.index_cast %one : index to i64
    %cf = arith.sitofp %ci : i64 to f32
    return %cf : f32
  }

  // 2-D pooling forward, one thread per output element (N C P Q). Modes: max
  // 0 and 3, average including padding 1, excluding padding 2.
  gpu.func @agx_dnn_pool_fwd(%x0: memref<1xf32, 1>, %y0: memref<1xf32, 1>,
      %total: index, %C: index, %H: index, %W: index, %P: index, %Q: index,
      %R: index, %S: index, %ph: index, %pw: index, %sh: index, %sw: index,
      %xs0: index, %xs1: index, %xs2: index, %xs3: index,
      %ys0: index, %ys1: index, %ys2: index, %ys3: index,
      %mode: index, %alpha: f32, %beta: f32) kernel {
    %big_x = arith.constant 4611686018427387904 : index
    %x = memref.reinterpret_cast %x0 to offset: [0], sizes: [%big_x], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %big_y = arith.constant 4611686018427387904 : index
    %y = memref.reinterpret_cast %y0 to offset: [0], sizes: [%big_y], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %k0 = arith.constant 0 : index
    %k1 = arith.constant 1 : index
    %k2 = arith.constant 2 : index
    %k3 = arith.constant 3 : index
    %f0 = arith.constant 0.0 : f32
    %i = gpu.global_id x
    %live = arith.cmpi ult, %i, %total : index
    scf.if %live {
      %n, %c, %p, %q = func.call @agx_dnn_coord(%i, %C, %P, %Q)
          : (index, index, index, index) -> (index, index, index, index)
      %yo = func.call @agx_dnn_dot4(%n, %c, %p, %q, %ys0, %ys1, %ys2, %ys3)
          : (index, index, index, index, index, index, index, index) -> index
      %m0 = arith.cmpi eq, %mode, %k0 : index
      %m3 = arith.cmpi eq, %mode, %k3 : index
      %is_max = arith.ori %m0, %m3 : i1
      %r = scf.if %is_max -> f32 {
        %br, %bs = func.call @agx_dnn_pool_argmax(%x0, %n, %c, %p, %q, %H, %W, %R, %S, %ph, %pw, %sh, %sw, %xs0, %xs1, %xs2, %xs3)
            : (memref<1xf32, 1>, index, index, index, index, index, index, index, index, index, index, index, index, index, index, index, index) -> (index, index)
        %none = arith.cmpi eq, %br, %R : index
        %v = scf.if %none -> f32 {
          scf.yield %f0 : f32
        } else {
          %h0a = arith.muli %p, %sh : index
          %h0 = arith.subi %h0a, %ph : index
          %w0a = arith.muli %q, %sw : index
          %w0 = arith.subi %w0a, %pw : index
          %h = arith.addi %h0, %br : index
          %w = arith.addi %w0, %bs : index
          %o = func.call @agx_dnn_dot4(%n, %c, %h, %w, %xs0, %xs1, %xs2, %xs3)
              : (index, index, index, index, index, index, index, index) -> index
          %mv = memref.load %x[%o] : memref<?xf32, strided<[1]>, 1>
          scf.yield %mv : f32
        }
        scf.yield %v : f32
      } else {
        %h0a = arith.muli %p, %sh : index
        %h0 = arith.subi %h0a, %ph : index
        %w0a = arith.muli %q, %sw : index
        %w0 = arith.subi %w0a, %pw : index
        %sum = scf.for %rr = %k0 to %R step %k1 iter_args(%acc = %f0) -> f32 {
          %h = arith.addi %h0, %rr : index
          %hin0 = arith.cmpi sge, %h, %k0 : index
          %hin1 = arith.cmpi slt, %h, %H : index
          %hin = arith.andi %hin0, %hin1 : i1
          %row = scf.for %ss = %k0 to %S step %k1 iter_args(%acc2 = %acc) -> f32 {
            %w = arith.addi %w0, %ss : index
            %win0 = arith.cmpi sge, %w, %k0 : index
            %win1 = arith.cmpi slt, %w, %W : index
            %win = arith.andi %win0, %win1 : i1
            %in = arith.andi %hin, %win : i1
            %a = scf.if %in -> f32 {
              %o = func.call @agx_dnn_dot4(%n, %c, %h, %w, %xs0, %xs1, %xs2, %xs3)
                  : (index, index, index, index, index, index, index, index) -> index
              %v = memref.load %x[%o] : memref<?xf32, strided<[1]>, 1>
              %t = arith.addf %acc2, %v : f32
              scf.yield %t : f32
            } else {
              scf.yield %acc2 : f32
            }
            scf.yield %a : f32
          }
          scf.yield %row : f32
        }
        %cnt = func.call @agx_dnn_pool_count(%p, %q, %H, %W, %R, %S, %ph, %pw, %sh, %sw, %mode)
            : (index, index, index, index, index, index, index, index, index, index, index) -> f32
        %avg = arith.divf %sum, %cnt : f32
        scf.yield %avg : f32
      }
      func.call @agx_dnn_blend(%y0, %yo, %r, %alpha, %beta) : (memref<1xf32, 1>, index, f32, f32, f32) -> ()
    }
    gpu.return
  }

  // 2-D pooling backward, one thread per input element (N C H W), gathering
  // the gradients of the windows that cover it (max: to the first maximum).
  gpu.func @agx_dnn_pool_bwd(%dy0: memref<1xf32, 1>, %x0: memref<1xf32, 1>,
      %dx0: memref<1xf32, 1>,
      %total: index, %C: index, %H: index, %W: index, %P: index, %Q: index,
      %R: index, %S: index, %ph: index, %pw: index, %sh: index, %sw: index,
      %xs0: index, %xs1: index, %xs2: index, %xs3: index,
      %dys0: index, %dys1: index, %dys2: index, %dys3: index,
      %dxs0: index, %dxs1: index, %dxs2: index, %dxs3: index,
      %mode: index, %alpha: f32, %beta: f32) kernel {
    %big_dy = arith.constant 4611686018427387904 : index
    %dy = memref.reinterpret_cast %dy0 to offset: [0], sizes: [%big_dy], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %big_x = arith.constant 4611686018427387904 : index
    %x = memref.reinterpret_cast %x0 to offset: [0], sizes: [%big_x], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %big_dx = arith.constant 4611686018427387904 : index
    %dx = memref.reinterpret_cast %dx0 to offset: [0], sizes: [%big_dx], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %k0 = arith.constant 0 : index
    %k1 = arith.constant 1 : index
    %k3 = arith.constant 3 : index
    %f0 = arith.constant 0.0 : f32
    %i = gpu.global_id x
    %live = arith.cmpi ult, %i, %total : index
    scf.if %live {
      %n, %c, %h, %w = func.call @agx_dnn_coord(%i, %C, %H, %W)
          : (index, index, index, index) -> (index, index, index, index)
      %dxo = func.call @agx_dnn_dot4(%n, %c, %h, %w, %dxs0, %dxs1, %dxs2, %dxs3)
          : (index, index, index, index, index, index, index, index) -> index
      %m0 = arith.cmpi eq, %mode, %k0 : index
      %m3 = arith.cmpi eq, %mode, %k3 : index
      %is_max = arith.ori %m0, %m3 : i1
      // windows p with p sh - ph <= h < p sh - ph + R
      %hp = arith.addi %h, %ph : index
      %wp = arith.addi %w, %pw : index
      %tlo_h = arith.subi %hp, %R : index
      %tlo_h1 = arith.addi %tlo_h, %k1 : index
      %tlo_w = arith.subi %wp, %S : index
      %tlo_w1 = arith.addi %tlo_w, %k1 : index
      %shm1 = arith.subi %sh, %k1 : index
      %swm1 = arith.subi %sw, %k1 : index
      %ch0 = arith.addi %tlo_h1, %shm1 : index
      %ch = arith.divsi %ch0, %sh : index
      %cw0 = arith.addi %tlo_w1, %swm1 : index
      %cw = arith.divsi %cw0, %sw : index
      %hneg = arith.cmpi sle, %tlo_h1, %k0 : index
      %wneg = arith.cmpi sle, %tlo_w1, %k0 : index
      %plo = arith.select %hneg, %k0, %ch : index
      %qlo = arith.select %wneg, %k0, %cw : index
      %phi0 = arith.divsi %hp, %sh : index
      %qhi0 = arith.divsi %wp, %sw : index
      %phi1 = arith.addi %phi0, %k1 : index
      %qhi1 = arith.addi %qhi0, %k1 : index
      %phi = arith.minsi %phi1, %P : index
      %qhi = arith.minsi %qhi1, %Q : index
      %sum = scf.for %p = %plo to %phi step %k1 iter_args(%acc = %f0) -> f32 {
        %row = scf.for %q = %qlo to %qhi step %k1 iter_args(%acc2 = %acc) -> f32 {
          %dyo = func.call @agx_dnn_dot4(%n, %c, %p, %q, %dys0, %dys1, %dys2, %dys3)
              : (index, index, index, index, index, index, index, index) -> index
          %g = memref.load %dy[%dyo] : memref<?xf32, strided<[1]>, 1>
          %t = scf.if %is_max -> f32 {
            %br, %bs = func.call @agx_dnn_pool_argmax(%x0, %n, %c, %p, %q, %H, %W, %R, %S, %ph, %pw, %sh, %sw, %xs0, %xs1, %xs2, %xs3)
                : (memref<1xf32, 1>, index, index, index, index, index, index, index, index, index, index, index, index, index, index, index, index) -> (index, index)
            %h0a = arith.muli %p, %sh : index
            %h0 = arith.subi %h0a, %ph : index
            %w0a = arith.muli %q, %sw : index
            %w0 = arith.subi %w0a, %pw : index
            %mh = arith.addi %h0, %br : index
            %mw = arith.addi %w0, %bs : index
            %eh = arith.cmpi eq, %mh, %h : index
            %ew = arith.cmpi eq, %mw, %w : index
            %here = arith.andi %eh, %ew : i1
            %v = arith.select %here, %g, %f0 : f32
            %a = arith.addf %acc2, %v : f32
            scf.yield %a : f32
          } else {
            %cnt = func.call @agx_dnn_pool_count(%p, %q, %H, %W, %R, %S, %ph, %pw, %sh, %sw, %mode)
                : (index, index, index, index, index, index, index, index, index, index, index) -> f32
            %v = arith.divf %g, %cnt : f32
            %a = arith.addf %acc2, %v : f32
            scf.yield %a : f32
          }
          scf.yield %t : f32
        }
        scf.yield %row : f32
      }
      func.call @agx_dnn_blend(%dx0, %dxo, %sum, %alpha, %beta) : (memref<1xf32, 1>, index, f32, f32, f32) -> ()
    }
    gpu.return
  }

  // The parameter index of element (c, h, w): c (spatial) or c H W + h W + w
  // (per activation).
  func.func private @agx_dnn_bn_param(%c: index, %h: index, %w: index, %H: index, %W: index, %spatial: index) -> index {
    %k0 = arith.constant 0 : index
    %sp = arith.cmpi ne, %spatial, %k0 : index
    %ch = arith.muli %c, %H : index
    %chh = arith.addi %ch, %h : index
    %chw = arith.muli %chh, %W : index
    %pa = arith.addi %chw, %w : index
    %r = arith.select %sp, %c, %pa : index
    return %r : index
  }

  // y = alpha (scale (x - mean) invstd + bias) + beta y, with invstd =
  // 1 / sqrt(var + eps), or var itself when invstd_given.
  gpu.func @agx_dnn_bn_apply(%x0: memref<1xf32, 1>, %y0: memref<1xf32, 1>,
      %scale0: memref<1xf32, 1>, %bias0: memref<1xf32, 1>, %mean0: memref<1xf32, 1>,
      %var0: memref<1xf32, 1>, %total: index, %C: index, %H: index, %W: index,
      %xs0: index, %xs1: index, %xs2: index, %xs3: index,
      %ys0: index, %ys1: index, %ys2: index, %ys3: index,
      %spatial: index, %invstd_given: index, %eps: f32, %alpha: f32, %beta: f32) kernel {
    %big_x = arith.constant 4611686018427387904 : index
    %x = memref.reinterpret_cast %x0 to offset: [0], sizes: [%big_x], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %big_y = arith.constant 4611686018427387904 : index
    %y = memref.reinterpret_cast %y0 to offset: [0], sizes: [%big_y], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %big_sc = arith.constant 4611686018427387904 : index
    %sc = memref.reinterpret_cast %scale0 to offset: [0], sizes: [%big_sc], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %big_bi = arith.constant 4611686018427387904 : index
    %bi = memref.reinterpret_cast %bias0 to offset: [0], sizes: [%big_bi], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %big_me = arith.constant 4611686018427387904 : index
    %me = memref.reinterpret_cast %mean0 to offset: [0], sizes: [%big_me], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %big_va = arith.constant 4611686018427387904 : index
    %va = memref.reinterpret_cast %var0 to offset: [0], sizes: [%big_va], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %k0 = arith.constant 0 : index
    %f1 = arith.constant 1.0 : f32
    %i = gpu.global_id x
    %live = arith.cmpi ult, %i, %total : index
    scf.if %live {
      %n, %c, %h, %w = func.call @agx_dnn_coord(%i, %C, %H, %W)
          : (index, index, index, index) -> (index, index, index, index)
      %xo = func.call @agx_dnn_dot4(%n, %c, %h, %w, %xs0, %xs1, %xs2, %xs3)
          : (index, index, index, index, index, index, index, index) -> index
      %yo = func.call @agx_dnn_dot4(%n, %c, %h, %w, %ys0, %ys1, %ys2, %ys3)
          : (index, index, index, index, index, index, index, index) -> index
      %pi = func.call @agx_dnn_bn_param(%c, %h, %w, %H, %W, %spatial) : (index, index, index, index, index, index) -> index
      %v = memref.load %x[%xo] : memref<?xf32, strided<[1]>, 1>
      %s = memref.load %sc[%pi] : memref<?xf32, strided<[1]>, 1>
      %b = memref.load %bi[%pi] : memref<?xf32, strided<[1]>, 1>
      %m = memref.load %me[%pi] : memref<?xf32, strided<[1]>, 1>
      %vv = memref.load %va[%pi] : memref<?xf32, strided<[1]>, 1>
      %given = arith.cmpi ne, %invstd_given, %k0 : index
      %ve = arith.addf %vv, %eps : f32
      %sq = math.sqrt %ve : f32
      %inv = arith.divf %f1, %sq : f32
      %is = arith.select %given, %vv, %inv : f32
      %d = arith.subf %v, %m : f32
      %xh = arith.mulf %d, %is : f32
      %r = math.fma %s, %xh, %b : f32
      func.call @agx_dnn_blend(%y0, %yo, %r, %alpha, %beta) : (memref<1xf32, 1>, index, f32, f32, f32) -> ()
    }
    gpu.return
  }

  // The element offset of reduction element m of parameter pi: m runs over
  // (n, h, w) for spatial parameters and over n per activation.
  func.func private @agx_dnn_bn_elem(%pi: index, %m: index, %C: index, %H: index, %W: index, %spatial: index,
      %s0: index, %s1: index, %s2: index, %s3: index) -> index {
    %k0 = arith.constant 0 : index
    %sp = arith.cmpi ne, %spatial, %k0 : index
    %HW = arith.muli %H, %W : index
    // spatial: c = pi, (n, h, w) from m
    %sn = arith.divui %m, %HW : index
    %shw = arith.remui %m, %HW : index
    %sh = arith.divui %shw, %W : index
    %sw = arith.remui %shw, %W : index
    // per activation: n = m, (c, h, w) from pi
    %pc = arith.divui %pi, %HW : index
    %phw = arith.remui %pi, %HW : index
    %ph = arith.divui %phw, %W : index
    %pw = arith.remui %phw, %W : index
    %n = arith.select %sp, %sn, %m : index
    %c = arith.select %sp, %pi, %pc : index
    %h = arith.select %sp, %sh, %ph : index
    %w = arith.select %sp, %sw, %pw : index
    %o = func.call @agx_dnn_dot4(%n, %c, %h, %w, %s0, %s1, %s2, %s3)
        : (index, index, index, index, index, index, index, index) -> index
    return %o : index
  }

  // Batch statistics, one thread per parameter over M elements: the mean and
  // the biased variance (two passes). Writes save_mean and save_invstd =
  // 1 / sqrt(var + eps), and when update is set, running = (1 - factor)
  // running + factor batch, with the unbiased variance M / (M - 1) var.
  gpu.func @agx_dnn_bn_stats(%x0: memref<1xf32, 1>, %rmean0: memref<1xf32, 1>,
      %rvar0: memref<1xf32, 1>, %smean0: memref<1xf32, 1>, %sinv0: memref<1xf32, 1>,
      %params: index, %M: index, %C: index, %H: index, %W: index,
      %xs0: index, %xs1: index, %xs2: index, %xs3: index,
      %spatial: index, %update: index, %factor: f32, %eps: f32) kernel {
    %big_x = arith.constant 4611686018427387904 : index
    %x = memref.reinterpret_cast %x0 to offset: [0], sizes: [%big_x], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %big_rm = arith.constant 4611686018427387904 : index
    %rm = memref.reinterpret_cast %rmean0 to offset: [0], sizes: [%big_rm], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %big_rv = arith.constant 4611686018427387904 : index
    %rv = memref.reinterpret_cast %rvar0 to offset: [0], sizes: [%big_rv], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %big_sm = arith.constant 4611686018427387904 : index
    %sm = memref.reinterpret_cast %smean0 to offset: [0], sizes: [%big_sm], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %big_si = arith.constant 4611686018427387904 : index
    %si = memref.reinterpret_cast %sinv0 to offset: [0], sizes: [%big_si], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %k0 = arith.constant 0 : index
    %k1 = arith.constant 1 : index
    %f0 = arith.constant 0.0 : f32
    %f1 = arith.constant 1.0 : f32
    %pi = gpu.global_id x
    %live = arith.cmpi ult, %pi, %params : index
    scf.if %live {
      %sum = scf.for %m = %k0 to %M step %k1 iter_args(%acc = %f0) -> f32 {
        %o = func.call @agx_dnn_bn_elem(%pi, %m, %C, %H, %W, %spatial, %xs0, %xs1, %xs2, %xs3)
            : (index, index, index, index, index, index, index, index, index, index) -> index
        %v = memref.load %x[%o] : memref<?xf32, strided<[1]>, 1>
        %a = arith.addf %acc, %v : f32
        scf.yield %a : f32
      }
      %Mi = arith.index_cast %M : index to i64
      %Mf = arith.sitofp %Mi : i64 to f32
      %mean = arith.divf %sum, %Mf : f32
      %ss = scf.for %m = %k0 to %M step %k1 iter_args(%acc = %f0) -> f32 {
        %o = func.call @agx_dnn_bn_elem(%pi, %m, %C, %H, %W, %spatial, %xs0, %xs1, %xs2, %xs3)
            : (index, index, index, index, index, index, index, index, index, index) -> index
        %v = memref.load %x[%o] : memref<?xf32, strided<[1]>, 1>
        %d = arith.subf %v, %mean : f32
        %a = math.fma %d, %d, %acc : f32
        scf.yield %a : f32
      }
      %var = arith.divf %ss, %Mf : f32
      %ve = arith.addf %var, %eps : f32
      %sq = math.sqrt %ve : f32
      %inv = arith.divf %f1, %sq : f32
      memref.store %mean, %sm[%pi] : memref<?xf32, strided<[1]>, 1>
      memref.store %inv, %si[%pi] : memref<?xf32, strided<[1]>, 1>
      %up = arith.cmpi ne, %update, %k0 : index
      scf.if %up {
        %Mm1 = arith.subf %Mf, %f1 : f32
        %Mm1c = arith.maximumf %Mm1, %f1 : f32
        %unb = arith.divf %ss, %Mm1c : f32
        %omf = arith.subf %f1, %factor : f32
        %om = memref.load %rm[%pi] : memref<?xf32, strided<[1]>, 1>
        %ov = memref.load %rv[%pi] : memref<?xf32, strided<[1]>, 1>
        %fm = arith.mulf %mean, %factor : f32
        %nm = math.fma %om, %omf, %fm : f32
        %fv = arith.mulf %unb, %factor : f32
        %nv = math.fma %ov, %omf, %fv : f32
        memref.store %nm, %rm[%pi] : memref<?xf32, strided<[1]>, 1>
        memref.store %nv, %rv[%pi] : memref<?xf32, strided<[1]>, 1>
      }
    }
    gpu.return
  }

  // Batch normalization backward, parameter gradients, one thread per
  // parameter: sums[2 p] = sum dy, sums[2 p + 1] = sum dy xhat;
  // dscale = alpha sum dy xhat + beta dscale, dbias = alpha sum dy + beta dbias.
  gpu.func @agx_dnn_bn_bwd_params(%x0: memref<1xf32, 1>, %dy0: memref<1xf32, 1>,
      %smean0: memref<1xf32, 1>, %sinv0: memref<1xf32, 1>, %dscale0: memref<1xf32, 1>,
      %dbias0: memref<1xf32, 1>, %sums0: memref<1xf32, 1>,
      %params: index, %M: index, %C: index, %H: index, %W: index,
      %xs0: index, %xs1: index, %xs2: index, %xs3: index,
      %dys0: index, %dys1: index, %dys2: index, %dys3: index,
      %spatial: index, %alpha: f32, %beta: f32) kernel {
    %big_x = arith.constant 4611686018427387904 : index
    %x = memref.reinterpret_cast %x0 to offset: [0], sizes: [%big_x], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %big_dy = arith.constant 4611686018427387904 : index
    %dy = memref.reinterpret_cast %dy0 to offset: [0], sizes: [%big_dy], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %big_sm = arith.constant 4611686018427387904 : index
    %sm = memref.reinterpret_cast %smean0 to offset: [0], sizes: [%big_sm], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %big_si = arith.constant 4611686018427387904 : index
    %si = memref.reinterpret_cast %sinv0 to offset: [0], sizes: [%big_si], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %big_ds = arith.constant 4611686018427387904 : index
    %ds = memref.reinterpret_cast %dscale0 to offset: [0], sizes: [%big_ds], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %big_db = arith.constant 4611686018427387904 : index
    %db = memref.reinterpret_cast %dbias0 to offset: [0], sizes: [%big_db], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %big_sums = arith.constant 4611686018427387904 : index
    %sums = memref.reinterpret_cast %sums0 to offset: [0], sizes: [%big_sums], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %k0 = arith.constant 0 : index
    %k1 = arith.constant 1 : index
    %k2 = arith.constant 2 : index
    %f0 = arith.constant 0.0 : f32
    %pi = gpu.global_id x
    %live = arith.cmpi ult, %pi, %params : index
    scf.if %live {
      %mean = memref.load %sm[%pi] : memref<?xf32, strided<[1]>, 1>
      %inv = memref.load %si[%pi] : memref<?xf32, strided<[1]>, 1>
      %r:2 = scf.for %m = %k0 to %M step %k1 iter_args(%a = %f0, %b = %f0) -> (f32, f32) {
        %xo = func.call @agx_dnn_bn_elem(%pi, %m, %C, %H, %W, %spatial, %xs0, %xs1, %xs2, %xs3)
            : (index, index, index, index, index, index, index, index, index, index) -> index
        %dyo = func.call @agx_dnn_bn_elem(%pi, %m, %C, %H, %W, %spatial, %dys0, %dys1, %dys2, %dys3)
            : (index, index, index, index, index, index, index, index, index, index) -> index
        %v = memref.load %x[%xo] : memref<?xf32, strided<[1]>, 1>
        %g = memref.load %dy[%dyo] : memref<?xf32, strided<[1]>, 1>
        %d = arith.subf %v, %mean : f32
        %xh = arith.mulf %d, %inv : f32
        %na = arith.addf %a, %g : f32
        %nb = math.fma %g, %xh, %b : f32
        scf.yield %na, %nb : f32, f32
      }
      %p0 = arith.muli %pi, %k2 : index
      %p1 = arith.addi %p0, %k1 : index
      memref.store %r#0, %sums[%p0] : memref<?xf32, strided<[1]>, 1>
      memref.store %r#1, %sums[%p1] : memref<?xf32, strided<[1]>, 1>
      func.call @agx_dnn_blend(%dscale0, %pi, %r#1, %alpha, %beta) : (memref<1xf32, 1>, index, f32, f32, f32) -> ()
      func.call @agx_dnn_blend(%dbias0, %pi, %r#0, %alpha, %beta) : (memref<1xf32, 1>, index, f32, f32, f32) -> ()
    }
    gpu.return
  }

  // Batch normalization backward, data gradient, one thread per element:
  // dx = scale invstd / M (M dy - sum dy - xhat sum dy xhat).
  gpu.func @agx_dnn_bn_bwd_data(%x0: memref<1xf32, 1>, %dy0: memref<1xf32, 1>,
      %dx0: memref<1xf32, 1>, %scale0: memref<1xf32, 1>, %smean0: memref<1xf32, 1>,
      %sinv0: memref<1xf32, 1>, %sums0: memref<1xf32, 1>,
      %total: index, %M: index, %C: index, %H: index, %W: index,
      %xs0: index, %xs1: index, %xs2: index, %xs3: index,
      %dys0: index, %dys1: index, %dys2: index, %dys3: index,
      %dxs0: index, %dxs1: index, %dxs2: index, %dxs3: index,
      %spatial: index, %alpha: f32, %beta: f32) kernel {
    %big_x = arith.constant 4611686018427387904 : index
    %x = memref.reinterpret_cast %x0 to offset: [0], sizes: [%big_x], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %big_dy = arith.constant 4611686018427387904 : index
    %dy = memref.reinterpret_cast %dy0 to offset: [0], sizes: [%big_dy], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %big_dx = arith.constant 4611686018427387904 : index
    %dx = memref.reinterpret_cast %dx0 to offset: [0], sizes: [%big_dx], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %big_sc = arith.constant 4611686018427387904 : index
    %sc = memref.reinterpret_cast %scale0 to offset: [0], sizes: [%big_sc], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %big_sm = arith.constant 4611686018427387904 : index
    %sm = memref.reinterpret_cast %smean0 to offset: [0], sizes: [%big_sm], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %big_si = arith.constant 4611686018427387904 : index
    %si = memref.reinterpret_cast %sinv0 to offset: [0], sizes: [%big_si], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %big_sums = arith.constant 4611686018427387904 : index
    %sums = memref.reinterpret_cast %sums0 to offset: [0], sizes: [%big_sums], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %k1 = arith.constant 1 : index
    %k2 = arith.constant 2 : index
    %i = gpu.global_id x
    %live = arith.cmpi ult, %i, %total : index
    scf.if %live {
      %n, %c, %h, %w = func.call @agx_dnn_coord(%i, %C, %H, %W)
          : (index, index, index, index) -> (index, index, index, index)
      %xo = func.call @agx_dnn_dot4(%n, %c, %h, %w, %xs0, %xs1, %xs2, %xs3)
          : (index, index, index, index, index, index, index, index) -> index
      %dyo = func.call @agx_dnn_dot4(%n, %c, %h, %w, %dys0, %dys1, %dys2, %dys3)
          : (index, index, index, index, index, index, index, index) -> index
      %dxo = func.call @agx_dnn_dot4(%n, %c, %h, %w, %dxs0, %dxs1, %dxs2, %dxs3)
          : (index, index, index, index, index, index, index, index) -> index
      %pi = func.call @agx_dnn_bn_param(%c, %h, %w, %H, %W, %spatial) : (index, index, index, index, index, index) -> index
      %v = memref.load %x[%xo] : memref<?xf32, strided<[1]>, 1>
      %g = memref.load %dy[%dyo] : memref<?xf32, strided<[1]>, 1>
      %s = memref.load %sc[%pi] : memref<?xf32, strided<[1]>, 1>
      %mean = memref.load %sm[%pi] : memref<?xf32, strided<[1]>, 1>
      %inv = memref.load %si[%pi] : memref<?xf32, strided<[1]>, 1>
      %p0 = arith.muli %pi, %k2 : index
      %p1 = arith.addi %p0, %k1 : index
      %sdy = memref.load %sums[%p0] : memref<?xf32, strided<[1]>, 1>
      %sdyx = memref.load %sums[%p1] : memref<?xf32, strided<[1]>, 1>
      %Mi = arith.index_cast %M : index to i64
      %Mf = arith.sitofp %Mi : i64 to f32
      %d = arith.subf %v, %mean : f32
      %xh = arith.mulf %d, %inv : f32
      %mg = arith.mulf %g, %Mf : f32
      %t0 = arith.subf %mg, %sdy : f32
      %xs = arith.mulf %xh, %sdyx : f32
      %t1 = arith.subf %t0, %xs : f32
      %k = arith.mulf %s, %inv : f32
      %kM = arith.divf %k, %Mf : f32
      %r = arith.mulf %kM, %t1 : f32
      func.call @agx_dnn_blend(%dx0, %dxo, %r, %alpha, %beta) : (memref<1xf32, 1>, index, f32, f32, f32) -> ()
    }
    gpu.return
  }

  // f16 (kind 0) or bf16 (kind 1) bits to f32 and back (round to nearest
  // even; NaN stays a quiet NaN)
  func.func private @agx_dnn_h2f(%b: i16, %kind: index) -> f32 {
    %k0 = arith.constant 0 : index
    %c16 = arith.constant 16 : i32
    %is_h = arith.cmpi eq, %kind, %k0 : index
    %r = scf.if %is_h -> f32 {
      %h = arith.bitcast %b : i16 to f16
      %f = arith.extf %h : f16 to f32
      scf.yield %f : f32
    } else {
      %w = arith.extui %b : i16 to i32
      %s = arith.shli %w, %c16 : i32
      %f = arith.bitcast %s : i32 to f32
      scf.yield %f : f32
    }
    return %r : f32
  }
  func.func private @agx_dnn_f2h(%f: f32, %kind: index) -> i16 {
    %k0 = arith.constant 0 : index
    %c16 = arith.constant 16 : i32
    %c1 = arith.constant 1 : i32
    %round = arith.constant 32767 : i32
    %qnan = arith.constant 32704 : i16
    %is_h = arith.cmpi eq, %kind, %k0 : index
    %r = scf.if %is_h -> i16 {
      %h = arith.truncf %f : f32 to f16
      %b = arith.bitcast %h : f16 to i16
      scf.yield %b : i16
    } else {
      %bits = arith.bitcast %f : f32 to i32
      %hi = arith.shrui %bits, %c16 : i32
      %odd = arith.andi %hi, %c1 : i32
      %bias = arith.addi %round, %odd : i32
      %sum = arith.addi %bits, %bias : i32
      %top = arith.shrui %sum, %c16 : i32
      %b = arith.trunci %top : i32 to i16
      %nan = arith.cmpf uno, %f, %f : f32
      %v = arith.select %nan, %qnan, %b : i16
      scf.yield %v : i16
    }
    return %r : i16
  }

  // d = s for 4-D tensors with element strides, from f16 / bf16 (kind 0 / 1)
  // bits to f32 (to_f32) or back (from_f32). One thread per element.
  gpu.func @agx_dnn_to_f32(%s0: memref<1xi16, 1>, %d0: memref<1xf32, 1>,
      %total: index, %C: index, %H: index, %W: index,
      %ss0: index, %ss1: index, %ss2: index, %ss3: index,
      %ds0: index, %ds1: index, %ds2: index, %ds3: index, %kind: index) kernel {
    %big = arith.constant 4611686018427387904 : index
    %s = memref.reinterpret_cast %s0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xi16, 1> to memref<?xi16, strided<[1]>, 1>
    %d = memref.reinterpret_cast %d0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %i = gpu.global_id x
    %live = arith.cmpi ult, %i, %total : index
    scf.if %live {
      %so = func.call @agx_dnn_off(%i, %C, %H, %W, %ss0, %ss1, %ss2, %ss3)
          : (index, index, index, index, index, index, index, index) -> index
      %do = func.call @agx_dnn_off(%i, %C, %H, %W, %ds0, %ds1, %ds2, %ds3)
          : (index, index, index, index, index, index, index, index) -> index
      %b = memref.load %s[%so] : memref<?xi16, strided<[1]>, 1>
      %f = func.call @agx_dnn_h2f(%b, %kind) : (i16, index) -> f32
      memref.store %f, %d[%do] : memref<?xf32, strided<[1]>, 1>
    }
    gpu.return
  }
  gpu.func @agx_dnn_from_f32(%s0: memref<1xf32, 1>, %d0: memref<1xi16, 1>,
      %total: index, %C: index, %H: index, %W: index,
      %ss0: index, %ss1: index, %ss2: index, %ss3: index,
      %ds0: index, %ds1: index, %ds2: index, %ds3: index, %kind: index) kernel {
    %big = arith.constant 4611686018427387904 : index
    %s = memref.reinterpret_cast %s0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %d = memref.reinterpret_cast %d0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xi16, 1> to memref<?xi16, strided<[1]>, 1>
    %i = gpu.global_id x
    %live = arith.cmpi ult, %i, %total : index
    scf.if %live {
      %so = func.call @agx_dnn_off(%i, %C, %H, %W, %ss0, %ss1, %ss2, %ss3)
          : (index, index, index, index, index, index, index, index) -> index
      %do = func.call @agx_dnn_off(%i, %C, %H, %W, %ds0, %ds1, %ds2, %ds3)
          : (index, index, index, index, index, index, index, index) -> index
      %f = memref.load %s[%so] : memref<?xf32, strided<[1]>, 1>
      %b = func.call @agx_dnn_f2h(%f, %kind) : (f32, index) -> i16
      memref.store %b, %d[%do] : memref<?xi16, strided<[1]>, 1>
    }
    gpu.return
  }

  // ---- the graph API (zluda_dnn_agx/src/graph.rs)

  // erf by Abramowitz and Stegun 7.1.26 (|error| < 1.5e-7)
  func.func private @agx_dnn_erf(%x: f32) -> f32 {
    %f0 = arith.constant 0.0 : f32
    %f1 = arith.constant 1.0 : f32
    %p = arith.constant 0.3275911 : f32
    %a1 = arith.constant 0.254829592 : f32
    %a2 = arith.constant -0.284496736 : f32
    %a3 = arith.constant 1.421413741 : f32
    %a4 = arith.constant -1.453152027 : f32
    %a5 = arith.constant 1.061405429 : f32
    %nlog2e = arith.constant -1.44269504 : f32
    %ax = math.absf %x : f32
    %d = math.fma %p, %ax, %f1 : f32
    %t = arith.divf %f1, %d : f32
    %q0 = math.fma %a5, %t, %a4 : f32
    %q1 = math.fma %q0, %t, %a3 : f32
    %q2 = math.fma %q1, %t, %a2 : f32
    %q3 = math.fma %q2, %t, %a1 : f32
    %q4 = arith.mulf %q3, %t : f32
    %x2 = arith.mulf %ax, %ax : f32
    %e0 = arith.mulf %x2, %nlog2e : f32
    %e = math.exp2 %e0 fastmath<afn> : f32
    %qe = arith.mulf %q4, %e : f32
    %r = arith.subf %f1, %qe : f32
    %neg = arith.cmpf olt, %x, %f0 : f32
    %nr = arith.negf %r : f32
    %v = arith.select %neg, %nr, %r : f32
    return %v : f32
  }

  func.func private @agx_dnn_bool(%c: i1) -> f32 {
    %f0 = arith.constant 0.0 : f32
    %f1 = arith.constant 1.0 : f32
    %v = arith.select %c, %f1, %f0 : f32
    return %v : f32
  }

  // y = op(x, b, t) for cuDNN's pointwise modes (CUDNN_POINTWISE_*): p0, p1
  // and p2 are the relu lower clip, upper clip and slope, or the elu alpha
  // (p0), softplus and swish beta (p0). Backward modes take the forward
  // input as x and dy as b. NaN for unknown modes.
  func.func private @agx_dnn_pw_op(%mode: index, %x: f32, %b: f32, %t: f32, %p0: f32, %p1: f32, %p2: f32) -> f32 {
    %f0 = arith.constant 0.0 : f32
    %f1 = arith.constant 1.0 : f32
    %half = arith.constant 0.5 : f32
    %nan = arith.constant 0x7FC00000 : f32
    %rsqrt2 = arith.constant 0.707106781 : f32
    %k = arith.constant 0.797884561 : f32
    %c044 = arith.constant 0.044715 : f32
    %c134 = arith.constant 0.134145 : f32
    %rsqrt2pi = arith.constant 0.398942280 : f32
    %r = scf.index_switch %mode -> f32
    case 0 {
      %v = arith.addf %x, %b : f32
      scf.yield %v : f32
    }
    case 1 {
      %v = arith.mulf %x, %b : f32
      scf.yield %v : f32
    }
    case 2 {
      %v = arith.minnumf %x, %b : f32
      scf.yield %v : f32
    }
    case 3 {
      %v = arith.maxnumf %x, %b : f32
      scf.yield %v : f32
    }
    case 4 {
      %v = math.sqrt %x : f32
      scf.yield %v : f32
    }
    case 5 {
      %v = math.fma %b, %b, %x : f32
      scf.yield %v : f32
    }
    case 6 {
      %v = arith.divf %x, %b : f32
      scf.yield %v : f32
    }
    case 7 {
      %q = arith.divf %x, %b : f32
      %tq = math.trunc %q : f32
      %nb = arith.negf %b : f32
      %v = math.fma %tq, %nb, %x : f32
      scf.yield %v : f32
    }
    case 8 {
      %v = math.powf %x, %b : f32
      scf.yield %v : f32
    }
    case 9 {
      %v = arith.subf %x, %b : f32
      scf.yield %v : f32
    }
    case 10 {
      %v = math.absf %x : f32
      scf.yield %v : f32
    }
    case 11 {
      %v = math.ceil %x : f32
      scf.yield %v : f32
    }
    case 12 {
      %v = math.cos %x : f32
      scf.yield %v : f32
    }
    case 13 {
      %v = math.exp %x : f32
      scf.yield %v : f32
    }
    case 14 {
      %v = math.floor %x : f32
      scf.yield %v : f32
    }
    case 15 {
      %v = math.log %x : f32
      scf.yield %v : f32
    }
    case 16 {
      %v = arith.negf %x : f32
      scf.yield %v : f32
    }
    case 17 {
      %s = math.sqrt %x : f32
      %v = arith.divf %f1, %s : f32
      scf.yield %v : f32
    }
    case 18 {
      %v = math.sin %x : f32
      scf.yield %v : f32
    }
    case 19 {
      %sn = math.sin %x : f32
      %cs = math.cos %x : f32
      %v = arith.divf %sn, %cs : f32
      scf.yield %v : f32
    }
    case 20 {
      %v = func.call @agx_dnn_erf(%x) : (f32) -> f32
      scf.yield %v : f32
    }
    case 21 {
      scf.yield %x : f32
    }
    case 22 {
      %v = arith.divf %f1, %x : f32
      scf.yield %v : f32
    }
    case 100 {
      %below = arith.cmpf ole, %x, %p0 : f32
      %above = arith.cmpf oge, %x, %p1 : f32
      %d = arith.subf %x, %p0 : f32
      %lo = math.fma %p2, %d, %p0 : f32
      %v0 = arith.select %above, %p1, %x : f32
      %v = arith.select %below, %lo, %v0 : f32
      scf.yield %v : f32
    }
    case 101 {
      %v = func.call @agx_dnn_tanh(%x) : (f32) -> f32
      scf.yield %v : f32
    }
    case 102 {
      %v = func.call @agx_dnn_sigmoid(%x) : (f32) -> f32
      scf.yield %v : f32
    }
    case 103 {
      %pos = arith.cmpf ogt, %x, %f0 : f32
      %e = func.call @agx_dnn_expm1(%x) : (f32) -> f32
      %ae = arith.mulf %p0, %e : f32
      %v = arith.select %pos, %x, %ae : f32
      scf.yield %v : f32
    }
    case 104 {
      %u = arith.mulf %x, %rsqrt2 : f32
      %e = func.call @agx_dnn_erf(%u) : (f32) -> f32
      %e1 = arith.addf %e, %f1 : f32
      %hx = arith.mulf %x, %half : f32
      %v = arith.mulf %hx, %e1 : f32
      scf.yield %v : f32
    }
    case 105 {
      // log(1 + e^(beta x)) / beta, x beyond 20 / beta linear
      %bx = arith.mulf %x, %p0 : f32
      %e = math.exp %bx fastmath<afn> : f32
      %e1 = arith.addf %e, %f1 : f32
      %l = math.log %e1 fastmath<afn> : f32
      %sp = arith.divf %l, %p0 : f32
      %lim = arith.constant 20.0 : f32
      %big = arith.cmpf ogt, %bx, %lim : f32
      %v = arith.select %big, %x, %sp : f32
      scf.yield %v : f32
    }
    case 106 {
      %bx = arith.mulf %x, %p0 : f32
      %s = func.call @agx_dnn_sigmoid(%bx) : (f32) -> f32
      %v = arith.mulf %x, %s : f32
      scf.yield %v : f32
    }
    case 107 {
      %x2 = arith.mulf %x, %x : f32
      %c = math.fma %x2, %c044, %f1 : f32
      %xc = arith.mulf %x, %c : f32
      %u = arith.mulf %xc, %k : f32
      %th = func.call @agx_dnn_tanh(%u) : (f32) -> f32
      %t1 = arith.addf %th, %f1 : f32
      %hx = arith.mulf %x, %half : f32
      %v = arith.mulf %hx, %t1 : f32
      scf.yield %v : f32
    }
    case 200 {
      %below = arith.cmpf ole, %x, %p0 : f32
      %above = arith.cmpf oge, %x, %p1 : f32
      %g0 = arith.select %above, %f0, %f1 : f32
      %g = arith.select %below, %p2, %g0 : f32
      %v = arith.mulf %g, %b : f32
      scf.yield %v : f32
    }
    case 201 {
      %th = func.call @agx_dnn_tanh(%x) : (f32) -> f32
      %t2 = arith.mulf %th, %th : f32
      %g = arith.subf %f1, %t2 : f32
      %v = arith.mulf %g, %b : f32
      scf.yield %v : f32
    }
    case 202 {
      %s = func.call @agx_dnn_sigmoid(%x) : (f32) -> f32
      %s1 = arith.subf %f1, %s : f32
      %g = arith.mulf %s, %s1 : f32
      %v = arith.mulf %g, %b : f32
      scf.yield %v : f32
    }
    case 203 {
      %pos = arith.cmpf ogt, %x, %f0 : f32
      %e = math.exp %x fastmath<afn> : f32
      %ae = arith.mulf %p0, %e : f32
      %g = arith.select %pos, %f1, %ae : f32
      %v = arith.mulf %g, %b : f32
      scf.yield %v : f32
    }
    case 204 {
      // Phi(x) + x phi(x)
      %u = arith.mulf %x, %rsqrt2 : f32
      %e = func.call @agx_dnn_erf(%u) : (f32) -> f32
      %e1 = arith.addf %e, %f1 : f32
      %cdf = arith.mulf %e1, %half : f32
      %x2 = arith.mulf %x, %x : f32
      %nh = arith.constant -0.5 : f32
      %ex = arith.mulf %x2, %nh : f32
      %pe = math.exp %ex fastmath<afn> : f32
      %pdf = arith.mulf %pe, %rsqrt2pi : f32
      %g = math.fma %x, %pdf, %cdf : f32
      %v = arith.mulf %g, %b : f32
      scf.yield %v : f32
    }
    case 205 {
      %bx = arith.mulf %x, %p0 : f32
      %g = func.call @agx_dnn_sigmoid(%bx) : (f32) -> f32
      %v = arith.mulf %g, %b : f32
      scf.yield %v : f32
    }
    case 206 {
      %bx = arith.mulf %x, %p0 : f32
      %s = func.call @agx_dnn_sigmoid(%bx) : (f32) -> f32
      %s1 = arith.subf %f1, %s : f32
      %ss = arith.mulf %s, %s1 : f32
      %g = math.fma %bx, %ss, %s : f32
      %v = arith.mulf %g, %b : f32
      scf.yield %v : f32
    }
    case 207 {
      %x2 = arith.mulf %x, %x : f32
      %c = math.fma %x2, %c044, %f1 : f32
      %xc = arith.mulf %x, %c : f32
      %u = arith.mulf %xc, %k : f32
      %th = func.call @agx_dnn_tanh(%u) : (f32) -> f32
      %t1 = arith.addf %th, %f1 : f32
      %t2 = arith.mulf %th, %th : f32
      %sech = arith.subf %f1, %t2 : f32
      // du/dx = k (1 + 3 0.044715 x^2)
      %dc = math.fma %x2, %c134, %f1 : f32
      %du = arith.mulf %dc, %k : f32
      %hx = arith.mulf %x, %half : f32
      %r1 = arith.mulf %sech, %du : f32
      %r2 = arith.mulf %hx, %r1 : f32
      %h1 = arith.mulf %t1, %half : f32
      %g = arith.addf %h1, %r2 : f32
      %v = arith.mulf %g, %b : f32
      scf.yield %v : f32
    }
    case 300 {
      %c = arith.cmpf oeq, %x, %b : f32
      %v = func.call @agx_dnn_bool(%c) : (i1) -> f32
      scf.yield %v : f32
    }
    case 301 {
      %c = arith.cmpf une, %x, %b : f32
      %v = func.call @agx_dnn_bool(%c) : (i1) -> f32
      scf.yield %v : f32
    }
    case 302 {
      %c = arith.cmpf ogt, %x, %b : f32
      %v = func.call @agx_dnn_bool(%c) : (i1) -> f32
      scf.yield %v : f32
    }
    case 303 {
      %c = arith.cmpf oge, %x, %b : f32
      %v = func.call @agx_dnn_bool(%c) : (i1) -> f32
      scf.yield %v : f32
    }
    case 304 {
      %c = arith.cmpf olt, %x, %b : f32
      %v = func.call @agx_dnn_bool(%c) : (i1) -> f32
      scf.yield %v : f32
    }
    case 305 {
      %c = arith.cmpf ole, %x, %b : f32
      %v = func.call @agx_dnn_bool(%c) : (i1) -> f32
      scf.yield %v : f32
    }
    case 400 {
      %cx = arith.cmpf une, %x, %f0 : f32
      %cb = arith.cmpf une, %b, %f0 : f32
      %c = arith.andi %cx, %cb : i1
      %v = func.call @agx_dnn_bool(%c) : (i1) -> f32
      scf.yield %v : f32
    }
    case 401 {
      %cx = arith.cmpf une, %x, %f0 : f32
      %cb = arith.cmpf une, %b, %f0 : f32
      %c = arith.ori %cx, %cb : i1
      %v = func.call @agx_dnn_bool(%c) : (i1) -> f32
      scf.yield %v : f32
    }
    case 402 {
      %c = arith.cmpf oeq, %x, %f0 : f32
      %v = func.call @agx_dnn_bool(%c) : (i1) -> f32
      scf.yield %v : f32
    }
    case 601 {
      %c = arith.cmpf une, %t, %f0 : f32
      %v = arith.select %c, %x, %b : f32
      scf.yield %v : f32
    }
    default {
      scf.yield %nan : f32
    }
    return %r : f32
  }

  // y = op(alpha1 x, alpha2 b, t) over y's 4-D shape, every input broadcast
  // through 0 strides. One thread per element of y.
  gpu.func @agx_dnn_pw_graph(%x0: memref<1xf32, 1>, %b0: memref<1xf32, 1>, %t0: memref<1xf32, 1>, %y0: memref<1xf32, 1>,
      %total: index, %C: index, %H: index, %W: index,
      %xs0: index, %xs1: index, %xs2: index, %xs3: index,
      %bs0: index, %bs1: index, %bs2: index, %bs3: index,
      %ts0: index, %ts1: index, %ts2: index, %ts3: index,
      %ys0: index, %ys1: index, %ys2: index, %ys3: index,
      %mode: index, %p0: f32, %p1: f32, %p2: f32, %alpha1: f32, %alpha2: f32) kernel {
    %big = arith.constant 4611686018427387904 : index
    %x = memref.reinterpret_cast %x0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %b = memref.reinterpret_cast %b0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %t = memref.reinterpret_cast %t0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %y = memref.reinterpret_cast %y0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %i = gpu.global_id x
    %live = arith.cmpi ult, %i, %total : index
    scf.if %live {
      %n, %c, %h, %w = func.call @agx_dnn_coord(%i, %C, %H, %W)
          : (index, index, index, index) -> (index, index, index, index)
      %xo = func.call @agx_dnn_dot4(%n, %c, %h, %w, %xs0, %xs1, %xs2, %xs3)
          : (index, index, index, index, index, index, index, index) -> index
      %bo = func.call @agx_dnn_dot4(%n, %c, %h, %w, %bs0, %bs1, %bs2, %bs3)
          : (index, index, index, index, index, index, index, index) -> index
      %to = func.call @agx_dnn_dot4(%n, %c, %h, %w, %ts0, %ts1, %ts2, %ts3)
          : (index, index, index, index, index, index, index, index) -> index
      %yo = func.call @agx_dnn_dot4(%n, %c, %h, %w, %ys0, %ys1, %ys2, %ys3)
          : (index, index, index, index, index, index, index, index) -> index
      %xv = memref.load %x[%xo] : memref<?xf32, strided<[1]>, 1>
      %bv = memref.load %b[%bo] : memref<?xf32, strided<[1]>, 1>
      %tv = memref.load %t[%to] : memref<?xf32, strided<[1]>, 1>
      %ax = arith.mulf %xv, %alpha1 : f32
      %ab = arith.mulf %bv, %alpha2 : f32
      %r = func.call @agx_dnn_pw_op(%mode, %ax, %ab, %tv, %p0, %p1, %p2)
          : (index, f32, f32, f32, f32, f32, f32) -> f32
      memref.store %r, %y[%yo] : memref<?xf32, strided<[1]>, 1>
    }
    gpu.return
  }

  // y = the reduction of x over the dimensions where y has extent 1 and x has
  // e_d > 1 (cudnnReduceTensorOp_t: add 0, mul 1, min 2, max 3, amax 4,
  // avg 5, norm1 6, norm2 7, mul without zeros 8). One thread per element
  // of y.
  gpu.func @agx_dnn_reduce_graph(%x0: memref<1xf32, 1>, %y0: memref<1xf32, 1>,
      %total: index, %C: index, %H: index, %W: index,
      %xs0: index, %xs1: index, %xs2: index, %xs3: index,
      %ys0: index, %ys1: index, %ys2: index, %ys3: index,
      %e0: index, %e1: index, %e2: index, %e3: index, %op: index) kernel {
    %big = arith.constant 4611686018427387904 : index
    %x = memref.reinterpret_cast %x0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %y = memref.reinterpret_cast %y0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %k0 = arith.constant 0 : index
    %k1 = arith.constant 1 : index
    %k2 = arith.constant 2 : index
    %k3 = arith.constant 3 : index
    %k4 = arith.constant 4 : index
    %k5 = arith.constant 5 : index
    %k6 = arith.constant 6 : index
    %k7 = arith.constant 7 : index
    %k8 = arith.constant 8 : index
    %f0 = arith.constant 0.0 : f32
    %f1 = arith.constant 1.0 : f32
    %inf = arith.constant 0x7F800000 : f32
    %ninf = arith.constant 0xFF800000 : f32
    %i = gpu.global_id x
    %live = arith.cmpi ult, %i, %total : index
    scf.if %live {
      %n, %c, %h, %w = func.call @agx_dnn_coord(%i, %C, %H, %W)
          : (index, index, index, index) -> (index, index, index, index)
      %base = func.call @agx_dnn_dot4(%n, %c, %h, %w, %xs0, %xs1, %xs2, %xs3)
          : (index, index, index, index, index, index, index, index) -> index
      %yo = func.call @agx_dnn_dot4(%n, %c, %h, %w, %ys0, %ys1, %ys2, %ys3)
          : (index, index, index, index, index, index, index, index) -> index
      %e01 = arith.muli %e0, %e1 : index
      %e23 = arith.muli %e2, %e3 : index
      %count = arith.muli %e01, %e23 : index
      %is_mul = arith.cmpi eq, %op, %k1 : index
      %is_mnz = arith.cmpi eq, %op, %k8 : index
      %one_init = arith.ori %is_mul, %is_mnz : i1
      %is_min = arith.cmpi eq, %op, %k2 : index
      %is_max = arith.cmpi eq, %op, %k3 : index
      %i0a = arith.select %one_init, %f1, %f0 : f32
      %i0b = arith.select %is_min, %inf, %i0a : f32
      %init = arith.select %is_max, %ninf, %i0b : f32
      %acc = scf.for %k = %k0 to %count step %k1 iter_args(%a = %init) -> f32 {
        %d3 = arith.remui %k, %e3 : index
        %q3 = arith.divui %k, %e3 : index
        %d2 = arith.remui %q3, %e2 : index
        %q2 = arith.divui %q3, %e2 : index
        %d1 = arith.remui %q2, %e1 : index
        %d0 = arith.divui %q2, %e1 : index
        %off = func.call @agx_dnn_dot4(%d0, %d1, %d2, %d3, %xs0, %xs1, %xs2, %xs3)
            : (index, index, index, index, index, index, index, index) -> index
        %o = arith.addi %base, %off : index
        %v = memref.load %x[%o] : memref<?xf32, strided<[1]>, 1>
        %av = math.absf %v : f32
        %r = scf.index_switch %op -> f32
        case 1 {
          %m = arith.mulf %a, %v : f32
          scf.yield %m : f32
        }
        case 2 {
          %m = arith.minnumf %a, %v : f32
          scf.yield %m : f32
        }
        case 3 {
          %m = arith.maxnumf %a, %v : f32
          scf.yield %m : f32
        }
        case 4 {
          %m = arith.maxnumf %a, %av : f32
          scf.yield %m : f32
        }
        case 6 {
          %m = arith.addf %a, %av : f32
          scf.yield %m : f32
        }
        case 7 {
          %m = math.fma %v, %v, %a : f32
          scf.yield %m : f32
        }
        case 8 {
          %z = arith.cmpf oeq, %v, %f0 : f32
          %v1 = arith.select %z, %f1, %v : f32
          %m = arith.mulf %a, %v1 : f32
          scf.yield %m : f32
        }
        default {
          %m = arith.addf %a, %v : f32
          scf.yield %m : f32
        }
        scf.yield %r : f32
      }
      %is_avg = arith.cmpi eq, %op, %k5 : index
      %is_n2 = arith.cmpi eq, %op, %k7 : index
      %cnt_i = arith.index_cast %count : index to i64
      %cnt = arith.uitofp %cnt_i : i64 to f32
      %avg = arith.divf %acc, %cnt : f32
      %rt = math.sqrt %acc : f32
      %r0 = arith.select %is_avg, %avg, %acc : f32
      %res = arith.select %is_n2, %rt, %r0 : f32
      memref.store %res, %y[%yo] : memref<?xf32, strided<[1]>, 1>
    }
    gpu.return
  }
}
