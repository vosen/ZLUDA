extern crate level_zero as l0;
extern crate level_zero_sys as l0_sys;
#[macro_use]
extern crate lazy_static;
#[cfg(test)]
extern crate cuda_driver_sys;
extern crate lz4;
#[cfg(test)]
#[macro_use]
extern crate paste;

#[allow(warnings)]
mod cuda;
mod cuda_impl;
pub(crate) mod r#impl;
