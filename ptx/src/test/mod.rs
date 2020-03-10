use super::ptx;


#[test]
fn empty() {
    assert!(ptx::ModuleParser::new().parse(
        ".version 6.5 .target sm_30, debug")
        .unwrap() == ());
}

#[test]
fn vector_add() {
    let vector_add = include_str!("vectorAdd_kernel64.ptx");
    assert!(ptx::ModuleParser::new().parse(vector_add).unwrap() == ());
}