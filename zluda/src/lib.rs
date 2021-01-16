extern crate level_zero as l0;
extern crate level_zero_sys as l0_sys;
#[macro_use]
extern crate lazy_static;
#[cfg(test)]
extern crate cuda_driver_sys;
#[cfg(test)]
#[macro_use]
extern crate paste;
extern crate ptx;

#[allow(warnings)]
pub mod cuda;
mod cuda_impl;
pub(crate) mod r#impl;
