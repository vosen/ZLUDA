#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub ptx);

pub mod ast;
pub use ast::Module as Module;

#[test]
fn version() {
    assert!(ptx::ModuleParser::new().parse("
        .version 6.5
        .target
        .address_size 64
    ").unwrap() == ());
}