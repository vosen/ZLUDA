use super::*;

pub(super) fn run<'a, 'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    directives: Vec<UnconditionalDirective>,
) -> Result<Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>>, TranslateError> {
    directives
        .into_iter()
        .map(|directive| run_directive(resolver, directive))
        .collect::<Result<Vec<_>, _>>()
}

fn run_directive<'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    directive: Directive2<
        ast::Instruction<ast::ParsedOperand<SpirvWord>>,
        ast::ParsedOperand<SpirvWord>,
    >,
) -> Result<Directive2<ast::Instruction<SpirvWord>, SpirvWord>, TranslateError> {
    Ok(match directive {
        Directive2::Variable(linking, var) => Directive2::Variable(linking, var),
        Directive2::Method(method) => Directive2::Method(run_method(resolver, method)?),
    })
}

fn run_method<'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    method: Function2<
        ast::Instruction<ast::ParsedOperand<SpirvWord>>,
        ast::ParsedOperand<SpirvWord>,
    >,
) -> Result<Function2<ast::Instruction<SpirvWord>, SpirvWord>, TranslateError> {
    let body = method
        .body
        .map(|statements| {
            let mut result = Vec::with_capacity(statements.len());
            for statement in statements {
                run_statement(resolver, &mut result, statement)?;
            }
            Ok::<_, TranslateError>(result)
        })
        .transpose()?;
    Ok(Function2 {
        return_arguments: method.return_arguments,
        name: method.name,
        input_arguments: method.input_arguments,
        body,
        import_as: method.import_as,
        tuning: method.tuning,
        linkage: method.linkage,
        is_kernel: method.is_kernel,
    })
}

fn run_statement<'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    result: &mut Vec<Statement<ast::Instruction<SpirvWord>, SpirvWord>>,
    statement: UnconditionalStatement,
) -> Result<(), TranslateError> {
    let mut visitor = FlattenArguments::new(resolver, result);
    let new_statement = statement.visit_map(&mut visitor)?;
    visitor.result.push(new_statement);
    Ok(())
}

struct FlattenArguments<'a, 'input> {
    result: &'a mut Vec<ExpandedStatement>,
    resolver: &'a mut GlobalStringIdentResolver2<'input>,
    post_stmts: Vec<ExpandedStatement>,
}

impl<'a, 'input> FlattenArguments<'a, 'input> {
    fn new(
        resolver: &'a mut GlobalStringIdentResolver2<'input>,
        result: &'a mut Vec<ExpandedStatement>,
    ) -> Self {
        FlattenArguments {
            result,
            resolver,
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
        type_space: Option<(&ast::Type, ast::StateSpace)>,
        _is_dst: bool,
    ) -> Result<SpirvWord, TranslateError> {
        let (type_, state_space) = if let Some((type_, state_space)) = type_space {
            (type_, state_space)
        } else {
            return Err(TranslateError::UntypedSymbol);
        };
        if state_space == ast::StateSpace::Reg {
            let (reg_type, reg_space) = self.resolver.get_typed(reg)?;
            if *reg_space != ast::StateSpace::Reg {
                return Err(error_mismatched_type());
            }
            let reg_scalar_type = match reg_type {
                ast::Type::Scalar(underlying_type) => *underlying_type,
                _ => return Err(error_mismatched_type()),
            };
            let reg_type = reg_type.clone();
            let id_constant_stmt = self
                .resolver
                .register_unnamed(Some((reg_type.clone(), ast::StateSpace::Reg)));
            self.result.push(Statement::Constant(ConstantDefinition {
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
            let id_add_result = self
                .resolver
                .register_unnamed(Some((reg_type, state_space)));
            self.result
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
            let id_constant_stmt = self.resolver.register_unnamed(Some((
                ast::Type::Scalar(ast::ScalarType::S64),
                ast::StateSpace::Reg,
            )));
            self.result.push(Statement::Constant(ConstantDefinition {
                dst: id_constant_stmt,
                typ: ast::ScalarType::S64,
                value: ast::ImmediateValue::S64(offset as i64),
            }));
            let dst = self
                .resolver
                .register_unnamed(Some((type_.clone(), state_space)));
            self.result.push(Statement::PtrAccess(PtrAccess {
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
        type_space: Option<(&ast::Type, ast::StateSpace)>,
    ) -> Result<SpirvWord, TranslateError> {
        let (scalar_t, state_space) =
            if let Some((ast::Type::Scalar(scalar), state_space)) = type_space {
                (*scalar, state_space)
            } else {
                return Err(TranslateError::UntypedSymbol);
            };
        let id = self
            .resolver
            .register_unnamed(Some((ast::Type::Scalar(scalar_t), state_space)));
        self.result.push(Statement::Constant(ConstantDefinition {
            dst: id,
            typ: scalar_t,
            value,
        }));
        Ok(id)
    }

    fn vec_member(
        &mut self,
        vector_ident: SpirvWord,
        member: u8,
        _type_space: Option<(&ast::Type, ast::StateSpace)>,
        is_dst: bool,
    ) -> Result<SpirvWord, TranslateError> {
        let (vector_width, scalar_type, space) = match self.resolver.get_typed(vector_ident)? {
            (ast::Type::Vector(vector_width, scalar_t), space) => {
                (*vector_width, *scalar_t, *space)
            }
            _ => return Err(error_mismatched_type()),
        };
        let temporary = self
            .resolver
            .register_unnamed(Some((scalar_type.into(), space)));
        if is_dst {
            self.post_stmts.push(Statement::VectorWrite(VectorWrite {
                scalar_type,
                vector_width,
                vector_dst: vector_ident,
                vector_src: vector_ident,
                scalar_src: temporary,
                member,
            }));
        } else {
            self.result.push(Statement::VectorRead(VectorRead {
                scalar_type,
                vector_width,
                scalar_dst: temporary,
                vector_src: vector_ident,
                member,
            }));
        }
        Ok(temporary)
    }

    fn vec_pack(
        &mut self,
        vector_elements: Vec<SpirvWord>,
        type_space: Option<(&ast::Type, ast::StateSpace)>,
        is_dst: bool,
        relaxed_type_check: bool,
    ) -> Result<SpirvWord, TranslateError> {
        let (width, scalar_t, state_space) = match type_space {
            Some((ast::Type::Vector(width, scalar_t), space)) => (*width, *scalar_t, space),
            _ => return Err(error_mismatched_type()),
        };
        let temporary_vector = self
            .resolver
            .register_unnamed(Some((ast::Type::Vector(width, scalar_t), state_space)));
        let statement = Statement::RepackVector(RepackVectorDetails {
            is_extract: is_dst,
            typ: scalar_t,
            packed: temporary_vector,
            unpacked: vector_elements,
            relaxed_type_check,
        });
        if is_dst {
            self.post_stmts.push(statement);
        } else {
            self.result.push(statement);
        }
        Ok(temporary_vector)
    }
}

impl<'a, 'b> ast::VisitorMap<ast::ParsedOperand<SpirvWord>, SpirvWord, TranslateError>
    for FlattenArguments<'a, 'b>
{
    fn visit(
        &mut self,
        args: ast::ParsedOperand<SpirvWord>,
        type_space: Option<(&ast::Type, ast::StateSpace)>,
        is_dst: bool,
        relaxed_type_check: bool,
    ) -> Result<SpirvWord, TranslateError> {
        match args {
            ast::ParsedOperand::Reg(r) => self.reg(r),
            ast::ParsedOperand::Imm(x) => self.immediate(x, type_space),
            ast::ParsedOperand::RegOffset(reg, offset) => {
                self.reg_offset(reg, offset, type_space, is_dst)
            }
            ast::ParsedOperand::VecMember(vec, member) => {
                self.vec_member(vec, member, type_space, is_dst)
            }
            ast::ParsedOperand::VecPack(vecs) => {
                self.vec_pack(vecs, type_space, is_dst, relaxed_type_check)
            }
        }
    }

    fn visit_ident(
        &mut self,
        name: SpirvWord,
        _type_space: Option<(&ast::Type, ast::StateSpace)>,
        _is_dst: bool,
        _relaxed_type_check: bool,
    ) -> Result<<SpirvWord as ast::Operand>::Ident, TranslateError> {
        self.reg(name)
    }
}

impl Drop for FlattenArguments<'_, '_> {
    fn drop(&mut self) {
        self.result.extend(self.post_stmts.drain(..));
    }
}
