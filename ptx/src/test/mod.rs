use super::ptx;

fn parse_and_assert(s: &str) {
    let mut errors = Vec::new();
    let ast = ptx::ModuleParser::new().parse(&mut errors, s).unwrap();
    assert!(errors.len() == 0);
}

#[test]
fn empty() {
    parse_and_assert(".version 6.5 .target sm_30, debug");
}

#[test]
fn vector_add() {
    let vector_add = include_str!("vectorAdd_kernel64.ptx");
    parse_and_assert(vector_add);
}
