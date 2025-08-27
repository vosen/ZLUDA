# Supported hardware

1. AMD GPU support?

     ZLUDA supports AMD Radeon RX 5000 series and newer GPUs (both desktop and integrated).
     Older consumer GPUs (Polaris, Vega, etc.) and server‑class GPUs are not supported; these architectures differ significantly from recent desktop GPUs and would require substantial engineering effort.
     We expect that the near-future unified GPU architecture (UDNA) will be more similar to desktop GPUs.

1. Intel GPU support?

    ZLUDA previously supported Intel GPUs, but not currently. It is possible to revive the Intel backend. The development team is focusing on high‑quality AMD GPU support and welcomes contributions.

1. NVIDIA GPU support?

    Unlikely to ever be on the roadmap, because NVIDIA users can use the original CUDA. That said, if someone wants to add support we are open to contributions.

1. Qualcomm GPU support?

    It would be interesting to have Qualcomm GPU support, but the development team is focusing on high‑quality AMD GPU support. We welcome contributions.

1. macOS support?

    Unlikely to ever happen. There is very little non‑deprecated CUDA software for macOS, and what remains will soon be unsupported.

1. ZLUDA on top of OpenCL or Vulkan?

    ZLUDA could be ported to OpenCL or Vulkan, but with significantly reduced functionality. This might be acceptable for a narrow use case, but it would not be as general‑purpose as using the native backend. Examples of features available with the current compilation path that are not exposed by either Vulkan or OpenCL:
   * Disabling FP contraction
   * Explicit alignment
   * Some subgroup and group operations
   * Bindless images
   * Pointer casts
   * Arbitrary virtual calls
   * Inline assembly
   * Rounding modes
   * Denormal modes
  
    Additionally, performance libraries (cuBLAS, cuDNN, etc.) cannot be easily mapped through Vulkan or OpenCL.