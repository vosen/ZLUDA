use super::read_test_file;
use ptx_parser::PtxError;
use std::error;
use std::str;

macro_rules! test_ptx_parse_fails {
    ($fn_name:ident, $expected_err:expr) => {
        paste::item! {
            #[test]
            fn [<$fn_name _llvm>]() -> Result<(), Box<dyn std::error::Error>> {
                let ptx = read_test_file!(concat!(stringify!($fn_name), ".ptx"));
                test_parse_fails(&ptx, $expected_err)
            }
        }
    };
}

test_ptx_parse_fails!(unrecognized_braces, vec![PtxError::UnrecognizedStatement("mov.u32 foo, {} {};")]);

fn test_parse_fails(
    ptx_text: &str,
    expected_errs: Vec<PtxError>,
) -> Result<(), Box<dyn error::Error>> {
    if let Err(actual_errs) = ptx_parser::parse_module_checked(ptx_text) {
        assert_eq!(actual_errs, expected_errs);
    } else {
        panic!("Unexpected success");
    }

    Ok(())
}
