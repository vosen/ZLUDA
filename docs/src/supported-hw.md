# Supported hardware

1. AMD GPU support?

     ZLUDA supports AMD Radeon RX 5000 series and newer GPUs (both desktop and integrated).
     Older GPUs (Polaris, Vega, etc.) are not supported, server GPUs are not supported; those GPUs are sufficiently different as to require significant effort

1. Intel GPU support?

    ZLUDA used to support Intel GPUs, but it does not anymore. It is possible to add an Intel GPU backend again. The development team is focusing on high‑quality AMD GPU support, but we welcome contributions.

1. NVIDIA GPU support?

    Unlikely to ever be on the roadmap, because NVIDIA users can use the original CUDA. That said, if someone wants to add support we are open to contributions.

1. Qualcomm GPU support?

    It would be interesting to have Qualcomm GPU support, but the development team is focusing on high‑quality AMD GPU support. We welcome contributions.

1. macOS support?

    macOS support is not likely to ever be on the roadmap. There is very little non‑deprecated CUDA software for macOS and what remains will soon be unsupported.

1. ZLUDA on top of OpenCL or Vulkan?

    ZLUDA could be ported to OpenCL or Vulkan, but with drastically reduced functionality. This might be acceptable for a narrow use case, but it would not be as general‑purpose as using the native backend. Here is a sample of hardware/software features available with the current compilation path that are not exposed by either Vulkan or OpenCL:
    * Disabling FP contraction
    * Explicit alignment
    * Some subgroup and group operators
    * Bindless images
    * Pointer casts
    * Arbitrary virtual calls
    * Inline assembly
    * Rounding modes
    * Denormal modes
  
    Additionally, performance libraries (cuBLAS, cuDNN, etc.) cannot be mapped through Vulkan or OpenCL.