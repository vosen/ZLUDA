# Supported software

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