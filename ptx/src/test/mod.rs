use crate::pass::{self, TranslateError};
use ptx_parser as ast;

mod spirv_run;

fn parse_and_assert(ptx_text: &str) {
    ast::parse_module_checked(ptx_text).unwrap();
}

fn compile_and_assert(ptx_text: &str) -> Result<(), TranslateError> {
    let ast = ast::parse_module_checked(ptx_text).unwrap();
    let attributes = pass::Attributes { clock_rate: 2124000 };
    crate::to_llvm_module(ast, attributes)?;
    Ok(())
}

#[test]
fn empty() {
    parse_and_assert(".version 6.5 .target sm_30, debug");
}

#[test]
fn operands_ptx() {
    let vector_add = include_str!("operands.ptx");
    parse_and_assert(vector_add);
}

#[test]
#[allow(non_snake_case)]
fn vectorAdd_kernel64_ptx() -> Result<(), TranslateError> {
    let vector_add = include_str!("vectorAdd_kernel64.ptx");
    compile_and_assert(vector_add)
}

#[test]
#[allow(non_snake_case)]
fn _Z9vectorAddPKfS0_Pfi_ptx() -> Result<(), TranslateError> {
    let vector_add = include_str!("_Z9vectorAddPKfS0_Pfi.ptx");
    compile_and_assert(vector_add)
}

#[test]
#[allow(non_snake_case)]
fn vectorAdd_11_ptx() -> Result<(), TranslateError> {
    let vector_add = include_str!("vectorAdd_11.ptx");
    compile_and_assert(vector_add)
}
