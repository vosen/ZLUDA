use ptx_parser as ast;
use std::{
    env, error,
    ffi::OsStr,
    fs::{self, File},
    io::Write,
    path::Path,
};

mod expand_operands;
mod insert_implicit_conversions;
mod normalize_basic_blocks;

#[macro_export]
macro_rules! test_pass {
    ($pass:expr, $test_name:ident) => {
        paste::item! {
            #[test]
            fn [<$test_name>]() -> Result<(), Box<dyn std::error::Error>> {
                use crate::test::read_test_file;
                let ptx = read_test_file!(concat!(stringify!($test_name), ".ptx"));
                let mut parts = ptx.split("// %%% output %%%");
                let ptx_in = parts.next().unwrap_or("").trim();
                let ptx_out = parts.next().unwrap_or("").trim();
                assert!(parts.next().is_none());
                let out_dir_suffix = std::path::Path::new(file!())
                    .parent()
                    .and_then(|p| p.file_name())
                    .unwrap();
                crate::pass::test::test_pass_assert(stringify!($test_name), out_dir_suffix, $pass, ptx_in, ptx_out)
            }
        }
    };
}
pub(crate) use test_pass;

use crate::pass::IdentEntry;

use super::{Directive2, Function2, GlobalStringIdentResolver2, SpirvWord, Statement};

fn directive2_vec_to_string(
    resolver: &GlobalStringIdentResolver2,
    directives: Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>>,
) -> String {
    directives
        .into_iter()
        .map(|d| directive_to_string(resolver, d) + "\n")
        .collect::<Vec<_>>()
        .join("")
}

fn directive_to_string(
    resolver: &GlobalStringIdentResolver2,
    directive: Directive2<ast::Instruction<SpirvWord>, SpirvWord>,
) -> String {
    match directive {
        Directive2::Variable(linking_directive, variable) => {
            let ld_string = if !linking_directive.is_empty() {
                format!("{} ", linking_directive)
            } else {
                "".to_string()
            };
            format!("{}{};", ld_string, variable)
        }
        Directive2::Method(function) => function_to_string(resolver, function),
    }
}

fn function_to_string(
    resolver: &GlobalStringIdentResolver2,
    function: Function2<ast::Instruction<SpirvWord>, SpirvWord>,
) -> String {
    if function.import_as.is_some()
        || function.tuning.len() > 0
        || function.flush_to_zero_f32
        || function.flush_to_zero_f16f64
        || function.rounding_mode_f32 != ast::RoundingMode::NearestEven
        || function.rounding_mode_f16f64 != ast::RoundingMode::NearestEven
    {
        todo!("Figure out some way of representing these in text");
    }

    let linkage = if !function.linkage.is_empty() {
        format!("{} ", function.linkage)
    } else {
        "".to_string()
    };

    let entry = if !function.is_kernel {
        format!(".func ")
    } else {
        format!(".entry ")
    };

    let return_arguments = if function.return_arguments.len() > 0 {
        let args = function
            .return_arguments
            .iter()
            .map(|arg| format!("{}", arg))
            .collect::<Vec<_>>()
            .join(", ");

        format!("({}) ", args)
    } else {
        "".to_string()
    };

    let input_arguments = function
        .input_arguments
        .iter()
        .map(|arg| format!("\n    {}", arg))
        .collect::<Vec<_>>()
        .join(",");

    let body = if let Some(stmts) = function.body {
        let stmt_strings = stmts
            .into_iter()
            .map(|stmt| format!("    {}\n", statement_to_string(resolver, stmt)))
            .collect::<Vec<_>>()
            .join("");
        format!("\n{{\n{}}}", stmt_strings)
    } else {
        format!(";")
    };

    format!(
        "{}{}{}{} ({}\n){}",
        linkage, entry, return_arguments, function.name, input_arguments, body
    )
}

struct StatementFormatter<'a> {
    resolver: &'a GlobalStringIdentResolver2<'a>,
    dst_strings: Vec<String>,
    other_args: Vec<SpirvWord>,
}

impl<'a> StatementFormatter<'a> {
    fn new(resolver: &'a GlobalStringIdentResolver2<'a>) -> Self {
        Self {
            resolver,
            dst_strings: Vec::new(),
            other_args: Vec::new(),
        }
    }

    fn format(&self, op: &str) -> String {
        let assign_temps = if self.dst_strings.len() > 0 {
            let temps = self.dst_strings.join(", ");
            format!("{} = ", temps)
        } else {
            "".to_string()
        };

        let args = self
            .other_args
            .iter()
            .map(|arg| format!(" {}", arg))
            .collect::<Vec<_>>()
            .join(",");

        format!("{}{}{};", assign_temps, op, args)
    }
}

impl<'a> ast::VisitorMap<SpirvWord, SpirvWord, ()> for StatementFormatter<'a> {
    fn visit(
        &mut self,
        arg: SpirvWord,
        _type_space: Option<(&ptx_parser::Type, ptx_parser::StateSpace)>,
        is_dst: bool,
        _relaxed_type_check: bool,
    ) -> Result<SpirvWord, ()> {
        if is_dst {
            if let Some(IdentEntry {
                name: None,
                type_space,
            }) = self.resolver.ident_map.get(&arg)
            {
                let type_string = if let Some((type_, state_space)) = type_space {
                    // We use the type_space from the resolver rather than from the operand, to avoid hiding implicit conversions
                    let state_space = match state_space {
                        ast::StateSpace::Generic => ".generic".to_string(),
                        _ => format!("{}", state_space),
                    };
                    format!("{}{} ", state_space, type_)
                } else {
                    "".to_string()
                };

                self.dst_strings.push(format!("{}{}", type_string, arg));

                return Ok(arg);
            }
        }

        self.other_args.push(arg);

        Ok(arg)
    }

    fn visit_ident(
        &mut self,
        arg: <SpirvWord as ptx_parser::Operand>::Ident,
        type_space: Option<(&ptx_parser::Type, ptx_parser::StateSpace)>,
        is_dst: bool,
        relaxed_type_check: bool,
    ) -> Result<<SpirvWord as ptx_parser::Operand>::Ident, ()> {
        self.visit(arg, type_space, is_dst, relaxed_type_check)
    }
}

fn statement_to_string(
    resolver: &GlobalStringIdentResolver2,
    stmt: Statement<ast::Instruction<SpirvWord>, SpirvWord>,
) -> String {
    let (op, visit_args) = match &stmt {
        Statement::Variable(var) => (format!("{}", var), true),
        Statement::Instruction(instr) => (format!("{}", instr), true),
        Statement::Conversion(conv) => (format!("{}", conv), true),
        Statement::Constant(constant) => (format!("{}", constant), true),
        Statement::RepackVector(repack) => (format!("{}", repack), true),
        Statement::Label(label) => (format!("{}:", label), false),
        _ => todo!(),
    };
    if visit_args {
        let mut args_formatter = StatementFormatter::new(resolver);
        stmt.visit_map(&mut args_formatter).unwrap();
        args_formatter.format(&op)
    } else {
        op
    }
}

fn test_pass_assert<F, D>(
    name: &str,
    pass_name: &OsStr,
    run_pass: F,
    ptx_in: &str,
    expected_ptx_out: &str,
) -> Result<(), Box<dyn error::Error>>
where
    F: FnOnce(ast::Module) -> D,
    D: std::fmt::Display,
{
    let (actual_ptx_out, errs) = ast::parse_module_checked(ptx_in)
        .map(|ast| {
            let result = run_pass(ast);
            (result.to_string(), vec![])
        })
        .unwrap_or_else(|errs| ("".to_string(), errs));
    for err in errs {
        eprintln!("{}", err);
    }
    compare_ptx(
        name,
        pass_name,
        ptx_in,
        actual_ptx_out.trim(),
        expected_ptx_out,
    );
    Ok(())
}

fn compare_ptx(
    name: &str,
    pass_name: &OsStr,
    ptx_in: &str,
    actual_ptx_out: &str,
    expected_ptx_out: &str,
) {
    if actual_ptx_out != expected_ptx_out {
        maybe_save_output(name, pass_name, ptx_in, actual_ptx_out);
        let comparison = pretty_assertions::StrComparison::new(expected_ptx_out, actual_ptx_out);
        panic!("assertion failed: `(left == right)`\n\n{}", comparison);
    }
    if actual_ptx_out == "" {
        maybe_save_output(name, pass_name, ptx_in, actual_ptx_out);
        panic!("missing expected output");
    }
}

fn maybe_save_output(name: &str, pass_name: &OsStr, ptx_in: &str, actual_ptx_out: &str) {
    let output_dir = env::var("TEST_PTX_PASS_FAIL_DIR");
    if let Ok(output_dir) = output_dir {
        let output_dir = Path::new(&output_dir).join(pass_name);
        fs::create_dir_all(&output_dir).unwrap();
        let output_file = output_dir.join(format!("{}.ptx", name));
        let mut output_file = File::create(output_file).unwrap();
        output_file.write_all(ptx_in.as_bytes()).unwrap();
        output_file.write_all(b"\n\n// %%% output %%%\n\n").unwrap();
        output_file.write_all(actual_ptx_out.as_bytes()).unwrap();
        if !actual_ptx_out.ends_with("\n") {
            output_file.write_all(b"\n").unwrap();
        }
    }
}
