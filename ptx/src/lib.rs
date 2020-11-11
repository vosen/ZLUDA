#[cfg(test)]
extern crate paste;
#[macro_use]
extern crate lalrpop_util;
#[macro_use]
extern crate quick_error;

extern crate bit_vec;
extern crate half;
#[cfg(test)]
extern crate level_zero as ze;
#[cfg(test)]
extern crate level_zero_sys as l0;
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

pub use crate::ptx::ModuleParser;
pub use lalrpop_util::lexer::Token;
pub use lalrpop_util::ParseError;
pub use rspirv::dr::Error as SpirvError;
pub use translate::to_spirv_module;
pub use translate::KernelInfo;
pub use translate::TranslateError;

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
