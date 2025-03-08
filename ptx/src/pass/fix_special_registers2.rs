use super::*;

pub(super) fn run<'a, 'input>(
    resolver: &'a mut GlobalStringIdentResolver2<'input>,
    special_registers: &'a SpecialRegistersMap2,
    directives: Vec<UnconditionalDirective>,
) -> Result<Vec<UnconditionalDirective>, TranslateError> {
    let mut result = Vec::with_capacity(SpecialRegistersMap2::len() + directives.len());
    let mut sreg_to_function =
        FxHashMap::with_capacity_and_hasher(SpecialRegistersMap2::len(), Default::default());
    SpecialRegistersMap2::foreach_declaration(
        resolver,
        |sreg, (return_arguments, name, input_arguments)| {
            result.push(UnconditionalDirective::Method(UnconditionalFunction {
                return_arguments,
                name,
                input_arguments,
                body: None,
                import_as: None,
                tuning: Vec::new(),
                linkage: ast::LinkingDirective::EXTERN,
                is_kernel: false,
                flush_to_zero_f32: false,
                flush_to_zero_f16f64: false,
                rounding_mode_f32: ptx_parser::RoundingMode::NearestEven,
                rounding_mode_f16f64: ptx_parser::RoundingMode::NearestEven,
            }));
            sreg_to_function.insert(sreg, name);
        },
    );
    let mut visitor = SpecialRegisterResolver {
        resolver,
        special_registers,
        sreg_to_function,
        result: Vec::new(),
    };
    for directive in directives.into_iter() {
        result.push(run_directive(&mut visitor, directive)?);
    }
    Ok(result)
}

fn run_directive<'a, 'input>(
    visitor: &mut SpecialRegisterResolver<'a, 'input>,
    directive: UnconditionalDirective,
) -> Result<UnconditionalDirective, TranslateError> {
    Ok(match directive {
        var @ Directive2::Variable(..) => var,
        Directive2::Method(method) => Directive2::Method(run_method(visitor, method)?),
    })
}

fn run_method<'a, 'input>(
    visitor: &mut SpecialRegisterResolver<'a, 'input>,
    method: UnconditionalFunction,
) -> Result<UnconditionalFunction, TranslateError> {
    let body = method
        .body
        .map(|statements| {
            let mut result = Vec::with_capacity(statements.len());
            for statement in statements {
                run_statement(visitor, &mut result, statement)?;
            }
            Ok::<_, TranslateError>(result)
        })
        .transpose()?;
    Ok(Function2 { body, ..method })
}

fn run_statement<'a, 'input>(
    visitor: &mut SpecialRegisterResolver<'a, 'input>,
    result: &mut Vec<UnconditionalStatement>,
    statement: UnconditionalStatement,
) -> Result<(), TranslateError> {
    let converted_statement = statement.visit_map(visitor)?;
    result.extend(visitor.result.drain(..));
    result.push(converted_statement);
    Ok(())
}

struct SpecialRegisterResolver<'a, 'input> {
    resolver: &'a mut GlobalStringIdentResolver2<'input>,
    special_registers: &'a SpecialRegistersMap2,
    sreg_to_function: FxHashMap<PtxSpecialRegister, SpirvWord>,
    result: Vec<UnconditionalStatement>,
}

impl<'a, 'b, 'input>
    ast::VisitorMap<ast::ParsedOperand<SpirvWord>, ast::ParsedOperand<SpirvWord>, TranslateError>
    for SpecialRegisterResolver<'a, 'input>
{
    fn visit(
        &mut self,
        operand: ast::ParsedOperand<SpirvWord>,
        _type_space: Option<(&ptx_parser::Type, ptx_parser::StateSpace)>,
        is_dst: bool,
        _relaxed_type_check: bool,
    ) -> Result<ast::ParsedOperand<SpirvWord>, TranslateError> {
        map_operand(operand, &mut |ident, vector_index| {
            self.replace_sreg(ident, vector_index, is_dst)
        })
    }

    fn visit_ident(
        &mut self,
        args: SpirvWord,
        _type_space: Option<(&ptx_parser::Type, ptx_parser::StateSpace)>,
        is_dst: bool,
        _relaxed_type_check: bool,
    ) -> Result<SpirvWord, TranslateError> {
        Ok(self.replace_sreg(args, None, is_dst)?.unwrap_or(args))
    }
}

impl<'a, 'b, 'input> SpecialRegisterResolver<'a, 'input> {
    fn replace_sreg(
        &mut self,
        name: SpirvWord,
        vector_index: Option<u8>,
        is_dst: bool,
    ) -> Result<Option<SpirvWord>, TranslateError> {
        if let Some(sreg) = self.special_registers.get(name) {
            if is_dst {
                return Err(error_mismatched_type());
            }
            let input_arguments = match (vector_index, sreg.get_function_input_type()) {
                (Some(idx), Some(inp_type)) => {
                    if inp_type != ast::ScalarType::U8 {
                        return Err(TranslateError::Unreachable);
                    }
                    let constant = self.resolver.register_unnamed(Some((
                        ast::Type::Scalar(inp_type),
                        ast::StateSpace::Reg,
                    )));
                    self.result.push(Statement::Constant(ConstantDefinition {
                        dst: constant,
                        typ: inp_type,
                        value: ast::ImmediateValue::U64(idx as u64),
                    }));
                    vec![(constant, ast::Type::Scalar(inp_type), ast::StateSpace::Reg)]
                }
                (None, None) => Vec::new(),
                _ => return Err(error_mismatched_type()),
            };
            let return_type = sreg.get_function_return_type();
            let fn_result = self
                .resolver
                .register_unnamed(Some((ast::Type::Scalar(return_type), ast::StateSpace::Reg)));
            let return_arguments = vec![(
                fn_result,
                ast::Type::Scalar(return_type),
                ast::StateSpace::Reg,
            )];
            let data = ast::CallDetails {
                uniform: false,
                return_arguments: return_arguments
                    .iter()
                    .map(|(_, typ, space)| (typ.clone(), *space))
                    .collect(),
                input_arguments: input_arguments
                    .iter()
                    .map(|(_, typ, space)| (typ.clone(), *space))
                    .collect(),
            };
            let arguments = ast::CallArgs::<ast::ParsedOperand<SpirvWord>> {
                return_arguments: return_arguments.iter().map(|(name, _, _)| *name).collect(),
                func: self.sreg_to_function[&sreg],
                input_arguments: input_arguments
                    .iter()
                    .map(|(name, _, _)| ast::ParsedOperand::Reg(*name))
                    .collect(),
            };
            self.result
                .push(Statement::Instruction(ast::Instruction::Call {
                    data,
                    arguments,
                }));
            Ok(Some(fn_result))
        } else {
            Ok(None)
        }
    }
}

pub fn map_operand<T: Copy, Err>(
    this: ast::ParsedOperand<T>,
    fn_: &mut impl FnMut(T, Option<u8>) -> Result<Option<T>, Err>,
) -> Result<ast::ParsedOperand<T>, Err> {
    Ok(match this {
        ast::ParsedOperand::Reg(ident) => {
            ast::ParsedOperand::Reg(fn_(ident, None)?.unwrap_or(ident))
        }
        ast::ParsedOperand::RegOffset(ident, offset) => {
            ast::ParsedOperand::RegOffset(fn_(ident, None)?.unwrap_or(ident), offset)
        }
        ast::ParsedOperand::Imm(imm) => ast::ParsedOperand::Imm(imm),
        ast::ParsedOperand::VecMember(ident, member) => match fn_(ident, Some(member))? {
            Some(ident) => ast::ParsedOperand::Reg(ident),
            None => ast::ParsedOperand::VecMember(ident, member),
        },
        ast::ParsedOperand::VecPack(idents) => ast::ParsedOperand::VecPack(
            idents
                .into_iter()
                .map(|ident| Ok(fn_(ident, None)?.unwrap_or(ident)))
                .collect::<Result<Vec<_>, _>>()?,
        ),
    })
}
