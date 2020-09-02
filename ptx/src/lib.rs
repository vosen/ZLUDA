#[cfg(test)]
extern crate paste;
#[macro_use]
extern crate lalrpop_util;
#[macro_use]
extern crate quick_error;

extern crate bit_vec;
#[cfg(test)]
extern crate level_zero as ze;
#[cfg(test)]
extern crate level_zero_sys as l0;
extern crate rspirv;
extern crate spirv_headers as spirv;
extern crate half;

#[cfg(test)]
extern crate spirv_tools_sys as spirv_tools;

lalrpop_mod!(
    #[allow(warnings)]
    ptx
);

pub mod ast;
#[cfg(test)]
mod test;
mod translate;

pub use lalrpop_util::ParseError as ParseError;
pub use lalrpop_util::lexer::Token as Token;
pub use crate::ptx::ModuleParser as ModuleParser;
pub use translate::to_spirv as to_spirv;
pub use rspirv::dr::Error as SpirvError;

pub(crate) fn without_none<T>(x: Vec<Option<T>>) -> Vec<T> {
    x.into_iter().filter_map(|x| x).collect()
}
