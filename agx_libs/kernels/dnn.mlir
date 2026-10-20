// Pointwise, softmax, pooling and batch normalization kernels of zluda_dnn_agx
// (cuDNN), in f32 for every AGX generation like sgemm.mlir. Tensors are 4-D
// N x C x H x W with arbitrary element strides (0 broadcasts); a thread's
// linear index runs over N C H W in that order. Outputs follow cuDNN's
// blending, y = alpha r + beta y, and y is not read when beta is 0.

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
    %e = math.exp2 %a : f32
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
    %e = math.exp2 %a : f32
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
        %e = math.exp2 %dl : f32
        %a = arith.addf %acc, %e : f32
        scf.yield %a : f32
      }
      %l2 = math.log2 %s : f32
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
        %e = math.exp2 %dl : f32
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
        %e = math.exp2 %yl : f32
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
}
