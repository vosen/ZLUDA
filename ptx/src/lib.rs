#[macro_use]
extern crate lalrpop_util;
#[macro_use]
extern crate quick_error;

extern crate bit_vec;
#[cfg(test)]
extern crate level_zero_sys as l0;
#[cfg(test)]
extern crate level_zero as ze;
extern crate rspirv;
extern crate spirv_headers as spirv;

lalrpop_mod!(ptx);

#[cfg(test)]
mod test;
mod translate;
pub mod ast;

pub use ast::Module as Module;
pub use translate::to_spirv as to_spirv;

pub(crate) fn without_none<T>(x: Vec<Option<T>>) -> Vec<T> {
    x.into_iter().filter_map(|x| x).collect()
}