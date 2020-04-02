pub struct Module<'a> {
    pub version: (u8, u8),
    pub functions: Vec<Function<'a>>
}

pub struct Function<'a> {
    pub kernel: bool,
    pub name: &'a str,
    pub args: Vec<Argument>,
    pub body: Vec<Statement<'a>>,
}

pub struct Argument {

}

pub enum Statement<'a> {
    Label(&'a str),
    Variable(Variable),
    Instruction(Instruction)
}

pub struct Variable {
    
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
    Ret
}