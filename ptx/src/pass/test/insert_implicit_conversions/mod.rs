use crate::pass::{test::directive2_vec_to_string, *};

use super::test_pass;

macro_rules! test_insert_implicit_conversions {
    ($test_name:ident) => {
        test_pass!(run_insert_implicit_conversions, $test_name);
    };
}

fn run_insert_implicit_conversions(ptx: ptx_parser::Module) -> String {
    // We run the minimal number of passes required to produce the input expected by insert_implicit_conversions
    let mut flat_resolver = GlobalStringIdentResolver2::new(SpirvWord(1));
    let mut scoped_resolver = ScopedResolver::new(&mut flat_resolver);
    let directives = normalize_identifiers2::run(&mut scoped_resolver, ptx.directives).unwrap();
    let directives = normalize_predicates2::run(&mut flat_resolver, directives).unwrap();
    let directives = expand_operands::run(&mut flat_resolver, directives).unwrap();
    let directives = insert_implicit_conversions2::run(&mut flat_resolver, directives).unwrap();
    directive2_vec_to_string(&flat_resolver, directives)
}

test_insert_implicit_conversions!(default);
test_insert_implicit_conversions!(default_reg_b32_reg_f16x2);
