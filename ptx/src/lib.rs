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

lalrpop_mod!(
    #[allow(dead_code)]
    #[allow(unused_imports)]
    ptx
);

pub mod ast;
#[cfg(test)]
mod test;
mod translate;

pub use ast::Module;
pub use translate::to_spirv;

pub(crate) fn without_none<T>(x: Vec<Option<T>>) -> Vec<T> {
    x.into_iter().filter_map(|x| x).collect()
}
