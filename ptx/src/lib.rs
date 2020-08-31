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

pub use ast::Module;
pub use translate::to_spirv;

pub(crate) fn without_none<T>(x: Vec<Option<T>>) -> Vec<T> {
    x.into_iter().filter_map(|x| x).collect()
}
