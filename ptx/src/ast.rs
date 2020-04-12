use std::convert::From;
use std::convert::Into;
use std::error::Error;
use std::mem;
use std::num::ParseIntError;

quick_error! {
    #[derive(Debug)]
    pub enum PtxError {
        ParseInt (err: ParseIntError) {
            from()
            display("{}", err)
            cause(err)
        }
    }
}

pub trait UnwrapWithVec<E, To> {
    fn unwrap_with(self, errs: &mut Vec<E>) -> To;
}

impl<R: Default, EFrom: std::convert::Into<EInto>, EInto> UnwrapWithVec<EInto, R>
    for Result<R, EFrom>
{
    fn unwrap_with(self, errs: &mut Vec<EInto>) -> R {
        self.unwrap_or_else(|e| {
            errs.push(e.into());
            R::default()
        })
    }
}

impl<
        R1: Default,
        EFrom1: std::convert::Into<EInto>,
        R2: Default,
        EFrom2: std::convert::Into<EInto>,
        EInto,
    > UnwrapWithVec<EInto, (R1, R2)> for (Result<R1, EFrom1>, Result<R2, EFrom2>)
{
    fn unwrap_with(self, errs: &mut Vec<EInto>) -> (R1, R2) {
        let (x, y) = self;
        let r1 = x.unwrap_with(errs);
        let r2 = y.unwrap_with(errs);
        (r1, r2)
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

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Type {
    Scalar(ScalarType),
    ExtendedScalar(ExtendedScalarType),
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
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

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum ExtendedScalarType {
    F16x2,
    Pred,
}

impl Default for ScalarType {
    fn default() -> Self {
        ScalarType::B8
    }
}

pub enum Statement<'a> {
    Label(&'a str),
    Variable(Variable<'a>),
    Instruction(Instruction),
}

pub struct Variable<'a> {
    pub space: StateSpace,
    pub v_type: Type,
    pub name: &'a str,
    pub count: Option<u32>,
}

pub enum StateSpace {
    Reg,
    Sreg,
    Const,
    Global,
    Local,
    Shared,
}

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
