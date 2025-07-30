This crate is a last resort Linux-specific solution.
Most of the time we can inject ourselves into a process by having users
set `LD_LIBRARY_PATH`.
Unfortunately, there is software out there which dynamically links to CUDA and
and CUDA performance libraries using RPATH. On Linux, dynamic linker operates
using approximately this algorithm:
* If path contains `/` treat the name as a (possibly relative) path and just use it
* Otherwise return the first that succeeds:
  * Library with this name already loaded into the process
  * Try paths in `DT_RPATH` (if `DT_RUNPATH` is not present)
  * Try paths in `LD_LIBRARY_PATH`
  * Try paths in `DT_RUNPATH`
  * Try system paths

In order to defeat `DT_RPATH` this library needs to be preloaded with `LD_PRELOAD`.
On initialization we also preload all the performance libraries. We also hijack
`dlopen` and on every call to `dlopen` that tries to open a CUDA library we
redirect it to our libraries

We also expose `zluda_dlopen_noredirect` for the purpose of tracing libraries
so they can load real underlying library and not just get redirected to themselves
