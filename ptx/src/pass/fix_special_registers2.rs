use super::*;

pub(super) fn run<'a, 'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    special_registers: &'a SpecialRegistersMap2,
    directives: Vec<UnconditionalDirective<'input>>,
) -> Result<Vec<UnconditionalDirective<'input>>, TranslateError> {
    let declarations = SpecialRegistersMap2::generate_declarations(resolver);
    let mut result = Vec::with_capacity(declarations.len() + directives.len());
    let mut sreg_to_function =
        FxHashMap::with_capacity_and_hasher(declarations.len(), Default::default());
    for (sreg, declaration) in declarations {
        let name = if let ast::MethodName::Func(name) = declaration.name {
            name
        } else {
            return Err(error_unreachable());
        };
        result.push(UnconditionalDirective::Method(UnconditionalFunction {
            func_decl: declaration,
            globals: Vec::new(),
            body: None,
            import_as: None,
            tuning: Vec::new(),
            linkage: ast::LinkingDirective::EXTERN,
        }));
        sreg_to_function.insert(sreg, name);
    }
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
    directive: UnconditionalDirective<'input>,
) -> Result<UnconditionalDirective<'input>, TranslateError> {
    Ok(match directive {
        var @ Directive2::Variable(..) => var,
        Directive2::Method(method) => Directive2::Method(run_method(visitor, method)?),
    })
}

fn run_method<'a, 'input>(
    visitor: &mut SpecialRegisterResolver<'a, 'input>,
    method: UnconditionalFunction<'input>,
) -> Result<UnconditionalFunction<'input>, TranslateError> {
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
    Ok(Function2 {
        func_decl: method.func_decl,
        globals: method.globals,
        body,
        import_as: method.import_as,
        tuning: method.tuning,
        linkage: method.linkage,
    })
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
        self.replace_sreg(args, None, is_dst)
    }
}

impl<'a, 'b, 'input> SpecialRegisterResolver<'a, 'input> {
    fn replace_sreg(
        &mut self,
        name: SpirvWord,
        vector_index: Option<u8>,
        is_dst: bool,
    ) -> Result<SpirvWord, TranslateError> {
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
            Ok(fn_result)
        } else {
            Ok(name)
        }
    }
}

pub fn map_operand<T, U, Err>(
    this: ast::ParsedOperand<T>,
    fn_: &mut impl FnMut(T, Option<u8>) -> Result<U, Err>,
) -> Result<ast::ParsedOperand<U>, Err> {
    Ok(match this {
        ast::ParsedOperand::Reg(ident) => ast::ParsedOperand::Reg(fn_(ident, None)?),
        ast::ParsedOperand::RegOffset(ident, offset) => {
            ast::ParsedOperand::RegOffset(fn_(ident, None)?, offset)
        }
        ast::ParsedOperand::Imm(imm) => ast::ParsedOperand::Imm(imm),
        ast::ParsedOperand::VecMember(ident, member) => {
            ast::ParsedOperand::Reg(fn_(ident, Some(member))?)
        }
        ast::ParsedOperand::VecPack(idents) => ast::ParsedOperand::VecPack(
            idents
                .into_iter()
                .map(|ident| fn_(ident, None))
                .collect::<Result<Vec<_>, _>>()?,
        ),
    })
}
