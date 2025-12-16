use crate::pass::{test::directive2_vec_to_string, *};

use super::test_pass;

macro_rules! test_normalize_basic_blocks {
    ($test_name:ident) => {
        test_pass!(run_normalize_basic_blocks, $test_name);
    };
}

fn run_normalize_basic_blocks(ptx: ptx_parser::Module) -> String {
    // We run the minimal number of passes required to produce the input expected by insert_implicit_conversions
    let mut flat_resolver = GlobalStringIdentResolver2::new(SpirvWord(1));
    let mut scoped_resolver = ScopedResolver::new(&mut flat_resolver);
    let directives = normalize_identifiers::run(&mut scoped_resolver, ptx.directives).unwrap();
    let directives = normalize_predicates::run(&mut flat_resolver, directives).unwrap();
    let directives = expand_operands::run(&mut flat_resolver, directives).unwrap();
    let directives = normalize_basic_blocks::run(&mut flat_resolver, directives).unwrap();
    directive2_vec_to_string(&flat_resolver, directives)
}

test_normalize_basic_blocks!(trap);
