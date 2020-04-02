#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(ptx);

mod test;
mod spirv;
pub mod ast;

pub use ast::Module as Module;
pub use spirv::translate as to_spirv;
