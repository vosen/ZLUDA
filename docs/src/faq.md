# FAQ

> [!WARNING]  
> For legal reasons we can't help you with the pre-rollback versions (older than 4). See more here: [https://www.theregister.com/2024/08/09/amd_zluda_take_down](https://www.theregister.com/2024/08/09/amd_zluda_take_down/)

## General

1. How I can donate to ZLUDA development?

    The ZLUDA project is fully funded and only accepts donations of labor.

1. What organization is funding ZLUDA development?

    This will be revealed in due time.

1. How can I follow ZLUDA's progress
   
   * Join our [Discord](https://discord.gg/sg6BNzXuc7)
   * Every quarter we publish a progress report on [ZLUDA's blog](https://vosen.github.io/ZLUDA/)


## Hardware

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

# Software

1. PyTorch support?

    PyTorch support is currently our top priority. We expect to have initial support fourth quarter of 2025.

1. Tensorflow support?

   Tensorflow support is currently a top priority for ZLUDA and will follow PyTorch support.

1. Blender support

    Blender is not on the roadmap, but it's often requested. Support might be added at certain point, but it's a low priority. If ZLUDA supports Blender, it will not support hardware ray-tracing (see _Hardware ray-tracing (OptiX) support?_ section below).

1. Hardware ray-tracing (OptiX) support?

    OptiX support is exceedingly complex. While it's built on top of CUDA, it uses its own dialect of PTX, uses its own host code and requires its own specific optimizations. It's unlikely that ZLUDA will ever support OptiX again. OptiX would require a very dedicated contributor (or team of contributors) to step in.

1. Support for games using 32 bit PhysX?

   We are convinced that it's possible (both for AMD GPUs and NVIDIA GPUs). Necessary groundwork has been done (log collection) and there is a plan how to implement the feature. It's not on the roadmap and we are hoping for outside contributors to step in.

1. Support for games using 64 bit PhysX (GameWorks)?

    It is definitely possible, pre-rollback ZLUDA had this capability. It's not on the roadmap and would require outside contributions.

1. DLSS support?

    Previously DLSS support was blocked by a missing functionality in AMD's Direct3D driver: ability to enqueue HIP kernels into Direct3D command list. This functionality now ships in the newest driver and DLSS support should be possible. It's not on our roadmap, but if someone steps in to implement it, we'd be happy to merge.