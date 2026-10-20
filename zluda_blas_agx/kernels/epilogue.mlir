// GEMM epilogues of zluda_blas_agx (cuBLASLt): D[i, j] = act(D[i, j] + bias[i])
// for column-major D, act none (0), ReLU (1) or GELU (2), in f32 for f32 and
// f16 D. One thread per element: launch an M x N grid (x along rows). For
// every AGX generation, like sgemm.mlir.

gpu.module @agx_blas_epilogue {
  gpu.func @agx_epilogue_f32(%d0: memref<1xf32, 1>, %bias0: memref<1xf32, 1>,
                            %M: index, %N: index, %ld: index,
                            %has_bias: index, %act: index) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %i2 = arith.constant 2 : index
    %big = arith.constant 4611686018427387904 : index
    %D = memref.reinterpret_cast %d0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %Bias = memref.reinterpret_cast %bias0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %i = gpu.global_id x
    %j = gpu.global_id y
    %in_m = arith.cmpi ult, %i, %M : index
    %in_n = arith.cmpi ult, %j, %N : index
    %in = arith.andi %in_m, %in_n : i1
    scf.if %in {
      %jl = arith.muli %j, %ld : index
      %p = arith.addi %jl, %i : index
      %x0 = memref.load %D[%p] : memref<?xf32, strided<[1]>, 1>
      %use_bias = arith.cmpi ne, %has_bias, %i0 : index
      %x1 = scf.if %use_bias -> f32 {
        %b = memref.load %Bias[%i] : memref<?xf32, strided<[1]>, 1>
        %s = arith.addf %x0, %b : f32
        scf.yield %s : f32
      } else {
        scf.yield %x0 : f32
      }
      %f0 = arith.constant 0.0 : f32
      %is_relu = arith.cmpi eq, %act, %i1 : index
      %is_gelu = arith.cmpi eq, %act, %i2 : index
      %relu = arith.maximumf %x1, %f0 : f32
      // GELU, tanh form: x / 2 * (1 + tanh(sqrt(2 / pi) * (x + 0.044715 x^3))),
      // with tanh(y) = 1 - 2 / (2^(2 y log2(e)) + 1)
      %c044 = arith.constant 0.044715 : f32
      %csqrt = arith.constant 0.7978845608 : f32
      %c2log2e = arith.constant 2.8853900818 : f32
      %f1 = arith.constant 1.0 : f32
      %f2 = arith.constant 2.0 : f32
      %fhalf = arith.constant 0.5 : f32
      %x2 = arith.mulf %x1, %x1 : f32
      %x3 = arith.mulf %x2, %x1 : f32
      %inner0 = math.fma %x3, %c044, %x1 : f32
      %y = arith.mulf %inner0, %csqrt : f32
      %e_arg = arith.mulf %y, %c2log2e : f32
      %e = math.exp2 %e_arg : f32
      %e1 = arith.addf %e, %f1 : f32
      %q = arith.divf %f2, %e1 : f32
      %tanh = arith.subf %f1, %q : f32
      %onep = arith.addf %tanh, %f1 : f32
      %hx = arith.mulf %x1, %fhalf : f32
      %gelu = arith.mulf %hx, %onep : f32
      %r0 = arith.select %is_relu, %relu, %x1 : f32
      %r = arith.select %is_gelu, %gelu, %r0 : f32
      memref.store %r, %D[%p] : memref<?xf32, strided<[1]>, 1>
    }
    gpu.return
  }
  gpu.func @agx_epilogue_f16(%d0: memref<1xf16, 1>, %bias0: memref<1xf16, 1>,
                            %M: index, %N: index, %ld: index,
                            %has_bias: index, %act: index) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %i2 = arith.constant 2 : index
    %big = arith.constant 4611686018427387904 : index
    %D = memref.reinterpret_cast %d0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf16, 1> to memref<?xf16, strided<[1]>, 1>
    %Bias = memref.reinterpret_cast %bias0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf16, 1> to memref<?xf16, strided<[1]>, 1>
    %i = gpu.global_id x
    %j = gpu.global_id y
    %in_m = arith.cmpi ult, %i, %M : index
    %in_n = arith.cmpi ult, %j, %N : index
    %in = arith.andi %in_m, %in_n : i1
    scf.if %in {
      %jl = arith.muli %j, %ld : index
      %p = arith.addi %jl, %i : index
      %xs = memref.load %D[%p] : memref<?xf16, strided<[1]>, 1>
      %x0 = arith.extf %xs : f16 to f32
      %use_bias = arith.cmpi ne, %has_bias, %i0 : index
      %x1 = scf.if %use_bias -> f32 {
        %bs = memref.load %Bias[%i] : memref<?xf16, strided<[1]>, 1>
        %b = arith.extf %bs : f16 to f32
        %s = arith.addf %x0, %b : f32
        scf.yield %s : f32
      } else {
        scf.yield %x0 : f32
      }
      %f0 = arith.constant 0.0 : f32
      %is_relu = arith.cmpi eq, %act, %i1 : index
      %is_gelu = arith.cmpi eq, %act, %i2 : index
      %relu = arith.maximumf %x1, %f0 : f32
      // GELU, tanh form: x / 2 * (1 + tanh(sqrt(2 / pi) * (x + 0.044715 x^3))),
      // with tanh(y) = 1 - 2 / (2^(2 y log2(e)) + 1)
      %c044 = arith.constant 0.044715 : f32
      %csqrt = arith.constant 0.7978845608 : f32
      %c2log2e = arith.constant 2.8853900818 : f32
      %f1 = arith.constant 1.0 : f32
      %f2 = arith.constant 2.0 : f32
      %fhalf = arith.constant 0.5 : f32
      %x2 = arith.mulf %x1, %x1 : f32
      %x3 = arith.mulf %x2, %x1 : f32
      %inner0 = math.fma %x3, %c044, %x1 : f32
      %y = arith.mulf %inner0, %csqrt : f32
      %e_arg = arith.mulf %y, %c2log2e : f32
      %e = math.exp2 %e_arg : f32
      %e1 = arith.addf %e, %f1 : f32
      %q = arith.divf %f2, %e1 : f32
      %tanh = arith.subf %f1, %q : f32
      %onep = arith.addf %tanh, %f1 : f32
      %hx = arith.mulf %x1, %fhalf : f32
      %gelu = arith.mulf %hx, %onep : f32
      %r0 = arith.select %is_relu, %relu, %x1 : f32
      %r = arith.select %is_gelu, %gelu, %r0 : f32
      %rs = arith.truncf %r : f32 to f16
      memref.store %rs, %D[%p] : memref<?xf16, strided<[1]>, 1>
    }
    gpu.return
  }
}
