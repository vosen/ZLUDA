#[derive(Clone)]
pub enum ParsedOperand<Ident> {
    Reg(Ident),
    RegOffset(Ident, i32),
    Imm(ImmediateValue),
    VecMember(Ident, u8),
    VecPack(Vec<Ident>),
}

#[derive(Copy, Clone)]
pub enum ImmediateValue {
    U64(u64),
    S64(i64),
    F32(f32),
    F64(f64),
}
