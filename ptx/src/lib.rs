#[cfg(test)]
extern crate paste;
#[macro_use]
extern crate lalrpop_util;

extern crate half;
#[cfg(test)]
extern crate hip_runtime_sys as hip;

lalrpop_mod!(ptx);

pub mod ast;
mod emit;
pub mod llvm;
pub mod raytracing;
#[cfg(test)]
mod test;
pub mod translate;

pub use crate::ptx::ModuleParser;
use ast::PtxError;
pub use lalrpop_util::lexer::Token;
pub use lalrpop_util::ParseError;
use std::fmt;
pub use translate::to_llvm_module;
pub use translate::to_llvm_module_for_raytracing;
pub use translate::Module;
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
                    sm_version: 0,
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

pub(crate) mod lalrpop {
    use crate::ast;
    use lalrpop_util::{lexer::Token, ParseError};

    enum ConstTypeMut<'a> {
        Type(&'a mut ast::Type),
        ArraySubtype(ast::ScalarType, &'a mut [u32]),
    }

    pub(crate) fn validate_variable_declaration2<'input>(
        variable: &mut ast::MultiVariableDefinition<&'input str>,
        errors: &mut Vec<ParseError<usize, Token<'input>, ast::PtxError>>,
    ) {
        if variable.variable.name == "_" {
            errors.push(ParseError::User {
                error: ast::PtxError::BlankVariableName,
            });
        }
        if let Some(suffix) = &mut variable.suffix {
            type_check_suffix(&mut variable.variable.type_, suffix, errors);
        }
        match variable.variable.type_ {
            ast::Type::Array(_, ref dims) => {
                if dims.len() > 1 && dims.contains(&0) {
                    errors.push(ParseError::User {
                        error: ast::PtxError::ZeroDimensionArray,
                    });
                }
            }
            _ => {}
        }
    }

    fn type_check_suffix<'input>(
        type_: &mut ast::Type,
        suffix: &mut ast::DeclarationSuffix<&'input str>,
        errors: &mut Vec<ParseError<usize, Token<'input>, ast::PtxError>>,
    ) {
        match suffix {
            ast::DeclarationSuffix::Count(_) => {
                if matches!(type_, ast::Type::Array(..)) {
                    errors.push(ParseError::User {
                        error: ast::PtxError::MultiArrayVariable,
                    });
                }
            }
            ast::DeclarationSuffix::Initializer(initializer) => {
                type_check_initializer(ConstTypeMut::Type(type_), initializer, errors);
            }
        }
    }

    fn type_check_initializer<'input>(
        type_: ConstTypeMut,
        initializer: &mut ast::Initializer<&'input str>,
        errors: &mut Vec<ParseError<usize, Token<'input>, ast::PtxError>>,
    ) {
        match (initializer, type_) {
            // Constant
            (
                ast::Initializer::Constant(immediate),
                ConstTypeMut::Type(ast::Type::Scalar(scalar)),
            ) => {
                if !immediate_is_type_compatible(immediate, *scalar) {
                    errors.push(ParseError::User {
                        error: ast::PtxError::InitializerTypeMismatch,
                    });
                }
            }
            // Array
            (
                ast::Initializer::Array(sub_inits),
                ConstTypeMut::Type(ast::Type::Array(ref mut scalar, dims)),
            ) => {
                type_check_pad_array(*scalar, sub_inits, dims, false, errors);
            }
            (
                ast::Initializer::Array(sub_inits),
                ConstTypeMut::ArraySubtype(ref mut scalar, dims),
            ) => {
                type_check_pad_array(*scalar, sub_inits, dims, true, errors);
            }
            (ast::Initializer::Array(..), _)
            | (_, ConstTypeMut::Type(ast::Type::Array(..)))
            | (_, ConstTypeMut::ArraySubtype(..)) => {
                errors.push(ParseError::User {
                    error: ast::PtxError::ArrayInitializer,
                });
            }
            // Global
            (ast::Initializer::Global(..), ConstTypeMut::Type(ast::Type::Scalar(scalar)))
            | (
                ast::Initializer::GenericGlobal(..),
                ConstTypeMut::Type(ast::Type::Scalar(scalar)),
            ) => check_global_type(*scalar, errors),
            // Add
            (ast::Initializer::Add(add), ConstTypeMut::Type(type_ @ ast::Type::Scalar(_))) => {
                type_check_initializer(ConstTypeMut::Type(type_), &mut add.0, errors);
                type_check_initializer(ConstTypeMut::Type(type_), &mut add.1, errors);
            }
            _ => {
                errors.push(ParseError::User {
                    error: ast::PtxError::InitializerTypeMismatch,
                });
            }
        }
    }

    fn immediate_is_type_compatible(
        immediate: &mut ast::ImmediateValue,
        scalar: ast::ScalarType,
    ) -> bool {
        let type_kind = scalar.kind();
        match immediate {
            ast::ImmediateValue::U64(_) | ast::ImmediateValue::S64(_) => {
                matches!(
                    type_kind,
                    ast::ScalarKind::Bit
                        | ast::ScalarKind::Signed
                        | ast::ScalarKind::Unsigned
                        | ast::ScalarKind::Pred
                )
            }
            ast::ImmediateValue::F32(_) | ast::ImmediateValue::F64(_) => {
                matches!(type_kind, ast::ScalarKind::Float)
            }
        }
    }

    fn type_check_pad_array<'input>(
        scalar: ast::ScalarType,
        initializers: &mut Vec<ast::Initializer<&'input str>>,
        dims: &mut [u32],
        subarray: bool,
        errors: &mut Vec<ParseError<usize, Token<'input>, ast::PtxError>>,
    ) {
        match dims.first_mut() {
            Some(x) => {
                if *x == 0 {
                    if subarray {
                        errors.push(ParseError::User {
                            error: ast::PtxError::ZeroDimensionArray,
                        });
                    } else {
                        *x = initializers.len() as u32;
                    }
                }
            }
            None => {
                errors.push(ParseError::User {
                    error: ast::PtxError::SyntaxError,
                });
                return;
            }
        }
        zero_pad_initializers(scalar, initializers, dims, errors);
        for initializer in initializers {
            if dims.len() == 1 {
                type_check_initializer(
                    ConstTypeMut::Type(&mut ast::Type::Scalar(scalar)),
                    initializer,
                    errors,
                );
            } else {
                type_check_initializer(
                    ConstTypeMut::ArraySubtype(scalar, &mut dims[1..]),
                    initializer,
                    errors,
                );
            }
        }
    }

    fn zero_pad_initializers<'input>(
        scalar: ast::ScalarType,
        initializers: &mut Vec<ast::Initializer<&str>>,
        dims: &[u32],
        errors: &mut Vec<ParseError<usize, Token<'input>, ast::PtxError>>,
    ) {
        let expected_len = dims[0] as usize;
        if expected_len == initializers.len() {
            return;
        } else if expected_len > initializers.len() {
            if dims.len() > 1 {
                initializers.resize_with(expected_len, || ast::Initializer::Array(Vec::new()));
            } else {
                initializers.resize_with(expected_len, || default_initializer_for(scalar));
            }
        } else {
            errors.push(ParseError::User {
                error: ast::PtxError::InitializerTypeMismatch,
            });
        }
    }

    fn default_initializer_for<'input>(scalar: ast::ScalarType) -> ast::Initializer<&'input str> {
        ast::Initializer::Constant(match scalar {
            ast::ScalarType::B8
            | ast::ScalarType::B16
            | ast::ScalarType::B32
            | ast::ScalarType::B64
            | ast::ScalarType::U8
            | ast::ScalarType::U16
            | ast::ScalarType::U32
            | ast::ScalarType::U64
            | ast::ScalarType::S8
            | ast::ScalarType::S16
            | ast::ScalarType::S32
            | ast::ScalarType::S64
            | ast::ScalarType::Pred => ast::ImmediateValue::U64(0),
            ast::ScalarType::F16
            | ast::ScalarType::F32
            | ast::ScalarType::F64
            | ast::ScalarType::F16x2 => ast::ImmediateValue::F64(0.0),
        })
    }

    fn check_global_type<'input>(
        scalar: ast::ScalarType,
        errors: &mut Vec<ParseError<usize, Token<'input>, ast::PtxError>>,
    ) {
        if scalar != ast::ScalarType::B64
            && scalar != ast::ScalarType::U64
            && scalar != ast::ScalarType::B64
            && scalar != ast::ScalarType::U64
        {
            errors.push(ParseError::User {
                error: ast::PtxError::InitializerTypeMismatch,
            });
        }
    }

    pub(crate) fn make_array_type<'input>(
        type_: ast::Type,
        array_dimensions: Option<Vec<u32>>,
        errors: &mut Vec<ParseError<usize, Token<'input>, ast::PtxError>>,
    ) -> ast::Type {
        match array_dimensions {
            Some(dimensions) => match type_ {
                ast::Type::Scalar(type_) => ast::Type::Array(type_, dimensions),
                _ => {
                    errors.push(ParseError::User {
                        error: ast::PtxError::NonScalarArray,
                    });
                    type_
                }
            },
            None => type_,
        }
    }

    pub(crate) fn validate_variable_declaration_func<'input>(
        variable: ast::VariableDeclaration<&'input str>,
        errors: &mut Vec<ParseError<usize, Token<'input>, ast::PtxError>>,
    ) -> ast::VariableDeclaration<&'input str> {
        validate_variable_declaration_impl(
            variable,
            errors,
            &[ast::StateSpace::Reg, ast::StateSpace::Param],
            false,
        )
    }

    pub(crate) fn validate_variable_declaration_entry<'input>(
        variable: ast::VariableDeclaration<&'input str>,
        errors: &mut Vec<ParseError<usize, Token<'input>, ast::PtxError>>,
    ) -> ast::VariableDeclaration<&'input str> {
        validate_variable_declaration_impl(variable, errors, &[ast::StateSpace::Param], false)
    }

    pub(crate) fn validate_variable_declaration_proto<'input>(
        variable: ast::VariableDeclaration<&'input str>,
        errors: &mut Vec<ParseError<usize, Token<'input>, ast::PtxError>>,
    ) -> ast::VariableDeclaration<&'input str> {
        validate_variable_declaration_impl(
            variable,
            errors,
            &[ast::StateSpace::Reg, ast::StateSpace::Param],
            true,
        )
    }

    pub(crate) fn validate_variable_declaration_impl<'input>(
        variable: ast::VariableDeclaration<&'input str>,
        errors: &mut Vec<ParseError<usize, Token<'input>, ast::PtxError>>,
        allowed: &[ast::StateSpace],
        allow_blank_name: bool,
    ) -> ast::VariableDeclaration<&'input str> {
        if !allowed.contains(&variable.state_space) {
            errors.push(ParseError::User {
                error: ast::PtxError::InvalidStateSpace,
            });
        }
        if !allow_blank_name && variable.name == "_" {
            errors.push(ParseError::User {
                error: ast::PtxError::BlankVariableName,
            });
        }
        variable
    }

    pub(crate) fn report_incorrect_variable<'input>(
        variable: &ast::VariableDeclaration<&'input str>,
        errors: &mut Vec<ParseError<usize, Token<'input>, ast::PtxError>>,
    ) {
        // Not mentioned in the docs
        if variable.state_space != ast::StateSpace::Reg
            && variable.type_ == ast::Type::Scalar(ast::ScalarType::Pred)
        {
            errors.push(ParseError::User {
                error: ast::PtxError::NonRegPredVariable,
            });
        }
        // "Vectors must be based on a fundamental type, and they may reside in the register space"
        // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#variable-declarations
        if variable.state_space != ast::StateSpace::Reg
            && matches!(variable.type_, ast::Type::Vector(..))
        {
            errors.push(ParseError::User {
                error: ast::PtxError::NonRegPredVariable,
            });
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
