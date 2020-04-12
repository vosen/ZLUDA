use std::convert::From;
use std::error::Error;
use std::mem;
use std::num::ParseIntError;

quick_error! {
    #[derive(Debug)]
    pub enum PtxError {
        Parse (err: ParseIntError) {
            display("{}", err)
            cause(err)
            from()
        }
    }
}

pub struct WithErrors<T, E> {
    pub value: T,
    pub errors: Vec<E>,
}

impl<T, E> WithErrors<T, E> {
    pub fn new(t: T) -> Self {
        WithErrors {
            value: t,
            errors: Vec::new(),
        }
    }

    pub fn map<F: FnOnce(T) -> U, U>(self, f: F) -> WithErrors<U, E> {
        WithErrors {
            value: f(self.value),
            errors: self.errors,
        }
    }

    pub fn map2<X, Y, F: FnOnce(X, Y) -> T>(
        x: WithErrors<X, E>,
        y: WithErrors<Y, E>,
        f: F,
    ) -> Self {
        let mut errors = x.errors;
        let mut errors_other = y.errors;
        if errors.len() < errors_other.len() {
            mem::swap(&mut errors, &mut errors_other);
        }
        errors.extend(errors_other);
        WithErrors {
            value: f(x.value, y.value),
            errors: errors,
        }
    }
}

impl<T:Default, E: Error> WithErrors<T, E> {
    pub fn from_results<X: Default, Y: Default, F: FnOnce(X, Y) -> T>(
        x: Result<X, E>,
        y: Result<Y, E>,
        f: F,
    ) -> Self {
        match (x, y) {
            (Ok(x), Ok(y)) => WithErrors {
                value: f(x, y),
                errors: Vec::new(),
            },
            (Err(e), Ok(y)) => WithErrors {
                value: f(X::default(), y),
                errors: vec![e],
            },
            (Ok(x), Err(e)) => WithErrors {
                value: f(x, Y::default()),
                errors: vec![e],
            },
            (Err(e1), Err(e2)) => WithErrors {
                value: T::default(),
                errors: vec![e1, e2],
            },
        }
    }
}

impl<T, E: Error> WithErrors<Vec<T>, E> {
    pub fn from_vec(v: Vec<WithErrors<T, E>>) -> Self {
        let mut values = Vec::with_capacity(v.len());
        let mut errors = Vec::new();
        for we in v.into_iter() {
            values.push(we.value);
            errors.extend(we.errors);
        }
        WithErrors {
            value: values,
            errors: errors,
        }
    }
}

pub trait WithErrorsExt<From, To, E> {
    fn with_errors<F: FnOnce(From) -> To>(self, f: F) -> WithErrors<To, E>;
}

impl<From, To: Default, E> WithErrorsExt<From, To, E> for Result<From, E> {
    fn with_errors<F: FnOnce(From) -> To>(self, f: F) -> WithErrors<To, E> {
        self.map_or_else(
            |e| WithErrors {
                value: To::default(),
                errors: vec![e],
            },
            |t| WithErrors {
                value: f(t),
                errors: Vec::new(),
            },
        )
    }
}

pub struct Module<'a> {
    pub version: (u8, u8),
    pub functions: Vec<Function<'a>>,
}

pub struct Function<'a> {
    pub kernel: bool,
    pub name: &'a str,
    pub args: Vec<Argument<'a>>,
    pub body: Vec<Statement<'a>>,
}

#[derive(Default)]
pub struct Argument<'a> {
    pub name: &'a str,
    pub a_type: ScalarType,
    pub length: u32,
}

pub enum ScalarType {
    B8,
    B16,
    B32,
    B64,
    U8,
    U16,
    U32,
    U64,
    S8,
    S16,
    S32,
    S64,
    F16,
    F32,
    F64,
}

impl Default for ScalarType {
    fn default() -> Self {
        ScalarType::B8
    }
}

pub enum Statement<'a> {
    Label(&'a str),
    Variable(Variable),
    Instruction(Instruction),
}

pub struct Variable {}

pub enum Instruction {
    Ld,
    Mov,
    Mul,
    Add,
    Setp,
    Not,
    Bra,
    Cvt,
    Shl,
    At,
    Ret,
}
