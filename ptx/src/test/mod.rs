use super::ptx;
use super::TranslateError;

mod spirv_run;

fn parse_and_assert(s: &str) {
    let mut errors = Vec::new();
    ptx::ModuleParser::new().parse(&mut errors, s).unwrap();
    assert!(errors.len() == 0);
}

fn compile_and_assert(s: &str) -> Result<(), TranslateError> {
    let mut errors = Vec::new();
    let ast = ptx::ModuleParser::new().parse(&mut errors, s).unwrap();
    crate::to_spirv_module(ast)?;
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
