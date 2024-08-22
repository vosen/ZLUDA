#[cfg(test)]
extern crate paste;
#[macro_use]
extern crate quick_error;
extern crate bit_vec;
extern crate half;
#[cfg(test)]
extern crate hip_runtime_sys as hip;
extern crate rspirv;
extern crate spirv_headers as spirv;

#[cfg(test)]
extern crate spirv_tools_sys as spirv_tools;

#[macro_use]
extern crate bitflags;

#[cfg(test)]
mod test;
mod translate;

pub use rspirv::dr::Error as SpirvError;
pub use translate::to_spirv_module;
pub use translate::KernelInfo;
pub use translate::TranslateError;
use ptx_parser as ast;
