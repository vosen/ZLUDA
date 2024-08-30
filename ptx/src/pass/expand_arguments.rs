use super::*;
use ptx_parser as ast;

pub(super) fn run<'a, 'b>(
    func: Vec<TypedStatement>,
    id_def: &'b mut MutableNumericIdResolver<'a>,
) -> Result<Vec<ExpandedStatement>, TranslateError> {
    let mut result = Vec::with_capacity(func.len());
    for s in func {
        match s {
            Statement::Label(id) => result.push(Statement::Label(id)),
            Statement::Conditional(bra) => result.push(Statement::Conditional(bra)),
            Statement::LoadVar(details) => result.push(Statement::LoadVar(details)),
            Statement::StoreVar(details) => result.push(Statement::StoreVar(details)),
            Statement::RetValue(d, id) => result.push(Statement::RetValue(d, id)),
            Statement::Conversion(conv) => result.push(Statement::Conversion(conv)),
            Statement::Constant(c) => result.push(Statement::Constant(c)),
            Statement::FunctionPointer(d) => result.push(Statement::FunctionPointer(d)),
            s => {
                let (new_statement, post_stmts) = {
                    let mut visitor = FlattenArguments::new(&mut result, id_def);
                    (s.visit_map(&mut visitor)?, visitor.post_stmts)
                };
                result.push(new_statement);
                result.extend(post_stmts);
            }
        }
    }
    Ok(result)
}

struct FlattenArguments<'a, 'b> {
    func: &'b mut Vec<ExpandedStatement>,
    id_def: &'b mut MutableNumericIdResolver<'a>,
    post_stmts: Vec<ExpandedStatement>,
}

impl<'a, 'b> FlattenArguments<'a, 'b> {
    fn new(
        func: &'b mut Vec<ExpandedStatement>,
        id_def: &'b mut MutableNumericIdResolver<'a>,
    ) -> Self {
        FlattenArguments {
            func,
            id_def,
            post_stmts: Vec::new(),
        }
    }

    fn reg(&mut self, name: SpirvWord) -> Result<SpirvWord, TranslateError> {
        Ok(name)
    }

    fn reg_offset(
        &mut self,
        reg: SpirvWord,
        offset: i32,
        type_space: Option<(&ptx_parser::Type, ptx_parser::StateSpace)>,
        _is_dst: bool,
    ) -> Result<SpirvWord, TranslateError> {
        let (type_, state_space) = if let Some((type_, state_space)) = type_space {
            (type_, state_space)
        } else {
            return Err(TranslateError::UntypedSymbol);
        };
        if state_space == ast::StateSpace::Reg || state_space == ast::StateSpace::Sreg {
            let (reg_type, reg_space) = self.id_def.get_typed(reg)?;
            if !state_is_compatible(reg_space, ast::StateSpace::Reg) {
                return Err(error_mismatched_type());
            }
            let reg_scalar_type = match reg_type {
                ast::Type::Scalar(underlying_type) => underlying_type,
                _ => return Err(error_mismatched_type()),
            };
            let id_constant_stmt = self
                .id_def
                .register_intermediate(reg_type.clone(), ast::StateSpace::Reg);
            self.func.push(Statement::Constant(ConstantDefinition {
                dst: id_constant_stmt,
                typ: reg_scalar_type,
                value: ast::ImmediateValue::S64(offset as i64),
            }));
            let arith_details = match reg_scalar_type.kind() {
                ast::ScalarKind::Signed => ast::ArithDetails::Integer(ast::ArithInteger {
                    type_: reg_scalar_type,
                    saturate: false,
                }),
                ast::ScalarKind::Unsigned | ast::ScalarKind::Bit => {
                    ast::ArithDetails::Integer(ast::ArithInteger {
                        type_: reg_scalar_type,
                        saturate: false,
                    })
                }
                _ => return Err(error_unreachable()),
            };
            let id_add_result = self.id_def.register_intermediate(reg_type, state_space);
            self.func
                .push(Statement::Instruction(ast::Instruction::Add {
                    data: arith_details,
                    arguments: ast::AddArgs {
                        dst: id_add_result,
                        src1: reg,
                        src2: id_constant_stmt,
                    },
                }));
            Ok(id_add_result)
        } else {
            let id_constant_stmt = self.id_def.register_intermediate(
                ast::Type::Scalar(ast::ScalarType::S64),
                ast::StateSpace::Reg,
            );
            self.func.push(Statement::Constant(ConstantDefinition {
                dst: id_constant_stmt,
                typ: ast::ScalarType::S64,
                value: ast::ImmediateValue::S64(offset as i64),
            }));
            let dst = self
                .id_def
                .register_intermediate(type_.clone(), state_space);
            self.func.push(Statement::PtrAccess(PtrAccess {
                underlying_type: type_.clone(),
                state_space: state_space,
                dst,
                ptr_src: reg,
                offset_src: id_constant_stmt,
            }));
            Ok(dst)
        }
    }

    fn immediate(
        &mut self,
        value: ast::ImmediateValue,
        type_space: Option<(&ptx_parser::Type, ptx_parser::StateSpace)>,
    ) -> Result<SpirvWord, TranslateError> {
        let (scalar_t, state_space) =
            if let Some((ast::Type::Scalar(scalar), state_space)) = type_space {
                (*scalar, state_space)
            } else {
                return Err(TranslateError::UntypedSymbol);
            };
        let id = self
            .id_def
            .register_intermediate(ast::Type::Scalar(scalar_t), state_space);
        self.func.push(Statement::Constant(ConstantDefinition {
            dst: id,
            typ: scalar_t,
            value,
        }));
        Ok(id)
    }
}

impl<'a, 'b> ast::VisitorMap<TypedOperand, SpirvWord, TranslateError> for FlattenArguments<'a, 'b> {
    fn visit(
        &mut self,
        args: TypedOperand,
        type_space: Option<(&ptx_parser::Type, ptx_parser::StateSpace)>,
        is_dst: bool,
        _relaxed_type_check: bool,
    ) -> Result<SpirvWord, TranslateError> {
        match args {
            TypedOperand::Reg(r) => self.reg(r),
            TypedOperand::Imm(x) => self.immediate(x, type_space),
            TypedOperand::RegOffset(reg, offset) => {
                self.reg_offset(reg, offset, type_space, is_dst)
            }
            TypedOperand::VecMember(..) => Err(error_unreachable()),
        }
    }

    fn visit_ident(
        &mut self,
        name: <TypedOperand as ptx_parser::Operand>::Ident,
        _type_space: Option<(&ptx_parser::Type, ptx_parser::StateSpace)>,
        _is_dst: bool,
        _relaxed_type_check: bool,
    ) -> Result<<SpirvWord as ptx_parser::Operand>::Ident, TranslateError> {
        self.reg(name)
    }
}
