#[cfg(test)]
extern crate paste;
#[macro_use]
extern crate lalrpop_util;
#[macro_use]
extern crate quick_error;

extern crate bit_vec;
extern crate half;
#[cfg(test)]
extern crate hip_runtime_sys as hip;
extern crate rspirv;
extern crate spirv_headers as spirv;

#[cfg(test)]
extern crate spirv_tools_sys as spirv_tools;

#[macro_use]
extern crate bitflags;

lalrpop_mod!(
    #[allow(warnings)]
    ptx
);

pub mod ast;
#[cfg(test)]
mod test;
mod translate;

use std::fmt;

pub use crate::ptx::ModuleParser;
use ast::PtxError;
pub use lalrpop_util::lexer::Token;
pub use lalrpop_util::ParseError;
pub use rspirv::dr::Error as SpirvError;
pub use translate::to_spirv_module;
pub use translate::KernelInfo;
pub use translate::TranslateError;

pub trait ModuleParserExt {
    fn parse_checked<'input>(
        txt: &'input str,
    ) -> Result<ast::Module<'input>, Vec<ParseError<usize, Token<'input>, ast::PtxError>>>;

    // Returned AST might be malformed. Some users, like logger, want to look at
    // malformed AST to record information - list of kernels or such
    fn parse_unchecked<'input>(
        txt: &'input str,
    ) -> (
        ast::Module<'input>,
        Vec<ParseError<usize, Token<'input>, ast::PtxError>>,
    );
}

impl ModuleParserExt for ModuleParser {
    fn parse_checked<'input>(
        txt: &'input str,
    ) -> Result<ast::Module<'input>, Vec<ParseError<usize, Token<'input>, ast::PtxError>>> {
        let mut errors = Vec::new();
        let maybe_ast = ptx::ModuleParser::new().parse(&mut errors, txt);
        match (&*errors, maybe_ast) {
            (&[], Ok(ast)) => Ok(ast),
            (_, Err(unrecoverable)) => {
                errors.push(unrecoverable);
                Err(errors)
            }
            (_, Ok(_)) => Err(errors),
        }
    }

    fn parse_unchecked<'input>(
        txt: &'input str,
    ) -> (
        ast::Module<'input>,
        Vec<ParseError<usize, Token<'input>, ast::PtxError>>,
    ) {
        let mut errors = Vec::new();
        let maybe_ast = ptx::ModuleParser::new().parse(&mut errors, txt);
        let ast = match maybe_ast {
            Ok(ast) => ast,
            Err(unrecoverable_err) => {
                errors.push(unrecoverable_err);
                ast::Module {
                    version: (0, 0),
                    directives: Vec::new(),
                }
            }
        };
        (ast, errors)
    }
}

pub struct DisplayParseError<'a, Loc, Tok, Err>(&'a str, &'a ParseError<Loc, Tok, Err>);

impl<'a, Loc: fmt::Display + Into<usize> + Copy, Tok, Err> DisplayParseError<'a, Loc, Tok, Err> {
    // unsafe because there's no guarantee that the input str is the one that this error was created from
    pub unsafe fn new(error: &'a ParseError<Loc, Tok, Err>, text: &'a str) -> Self {
        Self(text, error)
    }
}

impl<'a, Loc, Tok> fmt::Display for DisplayParseError<'a, Loc, Tok, PtxError>
where
    Loc: fmt::Display,
    Tok: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.1 {
            ParseError::User {
                error: PtxError::UnrecognizedStatement { start, end },
            } => self.fmt_unrecognized(f, *start, *end, "statement"),
            ParseError::User {
                error: PtxError::UnrecognizedDirective { start, end },
            } => self.fmt_unrecognized(f, *start, *end, "directive"),
            _ => self.1.fmt(f),
        }
    }
}

impl<'a, Loc, Tok, Err> DisplayParseError<'a, Loc, Tok, Err> {
    fn fmt_unrecognized(
        &self,
        f: &mut fmt::Formatter,
        start: usize,
        end: usize,
        kind: &'static str,
    ) -> fmt::Result {
        let full_substring = unsafe { self.0.get_unchecked(start..end) };
        write!(
            f,
            "Unrecognized {} `{}` found at {}:{}",
            kind, full_substring, start, end
        )
    }
}

pub(crate) fn without_none<T>(x: Vec<Option<T>>) -> Vec<T> {
    x.into_iter().filter_map(|x| x).collect()
}

pub(crate) fn vector_index<'input>(
    inp: &'input str,
) -> Result<u8, ParseError<usize, lalrpop_util::lexer::Token<'input>, ast::PtxError>> {
    match inp {
        "x" | "r" => Ok(0),
        "y" | "g" => Ok(1),
        "z" | "b" => Ok(2),
        "w" | "a" => Ok(3),
        _ => Err(ParseError::User {
            error: ast::PtxError::WrongVectorElement,
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::{DisplayParseError, ModuleParser, ModuleParserExt};

    #[test]
    fn error_report_unknown_instructions() {
        let module = r#"
            .version 6.5
            .target sm_30
            .address_size 64

            .visible .entry add(
                .param .u64 input,
            )
            {
                .reg .u64 	        x;
                does_not_exist.u64  x, x;
                ret;
            }"#;
        let errors = match ModuleParser::parse_checked(module) {
            Err(e) => e,
            Ok(_) => panic!(),
        };
        assert_eq!(errors.len(), 1);
        let reporter = DisplayParseError(module, &errors[0]);
        let build_log_string = format!("{}", reporter);
        assert!(build_log_string.contains("does_not_exist"));
    }
}
