use std::{
    env, error,
    fmt::Display,
    fs::{self, File},
    io::Write,
    path::Path,
};

mod insert_implicit_conversions;

#[macro_export]
macro_rules! test_pass {
    ($pass:expr, $test_name:ident) => {
        paste::item! {
            #[test]
            fn [<$test_name>]() -> Result<(), Box<dyn std::error::Error>> {
                use crate::test::read_test_file;
                let ptx = read_test_file!(concat!(stringify!($test_name), ".ptx"));
                let mut parts = ptx.split("// output:");
                let ptx_in = parts.next().unwrap_or("").trim();
                let ptx_out = parts.next().unwrap_or("").trim();
                assert!(parts.next().is_none());
                crate::pass::test::test_pass_assert(stringify!($test_name), $pass, ptx_in, ptx_out)
            }
        }
    };
}
pub(crate) use test_pass;

use super::Directive2;

struct DisplayDirective2Vec<Instruction, Operand: ptx_parser::Operand>(
    Vec<Directive2<Instruction, Operand>>,
);

impl<Instruction, Operand: ptx_parser::Operand> DisplayDirective2Vec<Instruction, Operand> {
    fn new(directives: Vec<Directive2<Instruction, Operand>>) -> Self {
        Self(directives)
    }
}

impl<Ident, Instruction, Operand> std::fmt::Display for DisplayDirective2Vec<Instruction, Operand>
where
    Ident: std::fmt::Display,
    Instruction: std::fmt::Display,
    Operand: ptx_parser::Operand<Ident = Ident>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for directive in &self.0 {
            write!(f, "{}", directive)?;
        }
        Ok(())
    }
}

fn test_pass_assert<F, D>(
    name: &str,
    run_pass: F,
    ptx_in: &str,
    expected_ptx_out: &str,
) -> Result<(), Box<dyn error::Error>>
where
    F: FnOnce(ptx_parser::Module) -> D,
    D: Display,
{
    let actual_ptx_out = ptx_parser::parse_module_checked(ptx_in)
        .map(|ast| {
            let result = run_pass(ast);
            result.to_string()
        })
        .unwrap_or("".to_string());
    compare_ptx(name, ptx_in, &actual_ptx_out, expected_ptx_out);
    Ok(())
}

fn compare_ptx(name: &str, ptx_in: &str, actual_ptx_out: &str, expected_ptx_out: &str) {
    if actual_ptx_out != expected_ptx_out {
        let output_dir = env::var("TEST_PTX_PASS_FAIL_DIR");
        if let Ok(output_dir) = output_dir {
            let output_dir = Path::new(&output_dir);
            fs::create_dir_all(&output_dir).unwrap();
            let output_file = output_dir.join(format!("{}.ptx", name));
            let mut output_file = File::create(output_file).unwrap();
            output_file.write_all(ptx_in.as_bytes()).unwrap();
            output_file.write_all(b"\n\n// output:\n\n").unwrap();
            output_file.write_all(actual_ptx_out.as_bytes()).unwrap();
        }
        let comparison = pretty_assertions::StrComparison::new(expected_ptx_out, actual_ptx_out);
        panic!("assertion failed: `(left == right)`\n\n{}", comparison);
    }
}
