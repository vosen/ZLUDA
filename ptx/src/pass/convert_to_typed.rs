use super::*;
use ptx_parser as ast;

pub(crate) fn run(
    func: Vec<UnconditionalStatement>,
    fn_defs: &GlobalFnDeclResolver,
    id_defs: &mut NumericIdResolver,
) -> Result<Vec<TypedStatement>, TranslateError> {
    let mut result = Vec::<TypedStatement>::with_capacity(func.len());
    for s in func {
        match s {
            Statement::Instruction(inst) => match inst {
                ast::Instruction::Mov {
                    data,
                    arguments:
                        ast::MovArgs {
                            dst: ast::ParsedOperand::Reg(dst_reg),
                            src: ast::ParsedOperand::Reg(src_reg),
                        },
                } if fn_defs.fns.contains_key(&src_reg) => {
                    if data.typ != ast::Type::Scalar(ast::ScalarType::U64) {
                        return Err(error_mismatched_type());
                    }
                    result.push(TypedStatement::FunctionPointer(FunctionPointerDetails {
                        dst: dst_reg,
                        src: src_reg,
                    }));
                }
                ast::Instruction::Call { data, arguments } => {
                    let resolver = fn_defs.get_fn_sig_resolver(arguments.func)?;
                    let resolved_call = resolver.resolve_in_spirv_repr(data, arguments)?;
                    let mut visitor = VectorRepackVisitor::new(&mut result, id_defs);
                    let reresolved_call =
                        Statement::Instruction(ast::visit_map(resolved_call, &mut visitor)?);
                    visitor.func.push(reresolved_call);
                    visitor.func.extend(visitor.post_stmts);
                }
                inst => {
                    let mut visitor = VectorRepackVisitor::new(&mut result, id_defs);
                    let instruction = Statement::Instruction(ast::visit_map(inst, &mut visitor)?);
                    visitor.func.push(instruction);
                    visitor.func.extend(visitor.post_stmts);
                }
            },
            Statement::Label(i) => result.push(Statement::Label(i)),
            Statement::Variable(v) => result.push(Statement::Variable(v)),
            Statement::Conditional(c) => result.push(Statement::Conditional(c)),
            _ => return Err(error_unreachable()),
        }
    }
    Ok(result)
}

struct VectorRepackVisitor<'a, 'b> {
    func: &'b mut Vec<TypedStatement>,
    id_def: &'b mut NumericIdResolver<'a>,
    post_stmts: Option<TypedStatement>,
}

impl<'a, 'b> VectorRepackVisitor<'a, 'b> {
    fn new(func: &'b mut Vec<TypedStatement>, id_def: &'b mut NumericIdResolver<'a>) -> Self {
        VectorRepackVisitor {
            func,
            id_def,
            post_stmts: None,
        }
    }

    fn convert_vector(
        &mut self,
        is_dst: bool,
        relaxed_type_check: bool,
        typ: &ast::Type,
        state_space: ast::StateSpace,
        idx: Vec<SpirvWord>,
    ) -> Result<SpirvWord, TranslateError> {
        // mov.u32 foobar, {a,b};
        let scalar_t = match typ {
            ast::Type::Vector(_, scalar_t) => *scalar_t,
            _ => return Err(error_mismatched_type()),
        };
        let temp_vec = self
            .id_def
            .register_intermediate(Some((typ.clone(), state_space)));
        let statement = Statement::RepackVector(RepackVectorDetails {
            is_extract: is_dst,
            typ: scalar_t,
            packed: temp_vec,
            unpacked: idx,
            relaxed_type_check,
        });
        if is_dst {
            self.post_stmts = Some(statement);
        } else {
            self.func.push(statement);
        }
        Ok(temp_vec)
    }
}

impl<'a, 'b> ast::VisitorMap<ast::ParsedOperand<SpirvWord>, TypedOperand, TranslateError>
    for VectorRepackVisitor<'a, 'b>
{
    fn visit_ident(
        &mut self,
        ident: SpirvWord,
        _: Option<(&ptx_parser::Type, ptx_parser::StateSpace)>,
        _: bool,
        _: bool,
    ) -> Result<SpirvWord, TranslateError> {
        Ok(ident)
    }

    fn visit(
        &mut self,
        op: ast::ParsedOperand<SpirvWord>,
        type_space: Option<(&ptx_parser::Type, ptx_parser::StateSpace)>,
        is_dst: bool,
        relaxed_type_check: bool,
    ) -> Result<TypedOperand, TranslateError> {
        Ok(match op {
            ast::ParsedOperand::Reg(reg) => TypedOperand::Reg(reg),
            ast::ParsedOperand::RegOffset(reg, offset) => TypedOperand::RegOffset(reg, offset),
            ast::ParsedOperand::Imm(x) => TypedOperand::Imm(x),
            ast::ParsedOperand::VecMember(vec, idx) => TypedOperand::VecMember(vec, idx),
            ast::ParsedOperand::VecPack(vec) => {
                let (type_, space) = type_space.ok_or_else(|| error_mismatched_type())?;
                TypedOperand::Reg(self.convert_vector(
                    is_dst,
                    relaxed_type_check,
                    type_,
                    space,
                    vec,
                )?)
            }
        })
    }
}
