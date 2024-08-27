use super::*;
use std::collections::HashMap;

pub(super) fn run<'a, 'b, 'input>(
    ptx_impl_imports: &'a mut HashMap<String, Directive<'input>>,
    typed_statements: Vec<TypedStatement>,
    numeric_id_defs: &'a mut NumericIdResolver<'b>,
) -> Result<Vec<TypedStatement>, TranslateError> {
    let result = Vec::with_capacity(typed_statements.len());
    let mut sreg_sresolver = SpecialRegisterResolver {
        ptx_impl_imports,
        numeric_id_defs,
        result,
    };
    for statement in typed_statements {
        let statement = statement.visit_map(&mut sreg_sresolver)?;
        sreg_sresolver.result.push(statement);
    }
    Ok(sreg_sresolver.result)
}

struct SpecialRegisterResolver<'a, 'b, 'input> {
    ptx_impl_imports: &'a mut HashMap<String, Directive<'input>>,
    numeric_id_defs: &'a mut NumericIdResolver<'b>,
    result: Vec<TypedStatement>,
}

impl<'a, 'b, 'input> ast::VisitorMap<TypedOperand, TypedOperand, TranslateError>
    for SpecialRegisterResolver<'a, 'b, 'input>
{
    fn visit(
        &mut self,
        operand: TypedOperand,
        _type_space: Option<(&ptx_parser::Type, ptx_parser::StateSpace)>,
        is_dst: bool,
        _relaxed_type_check: bool,
    ) -> Result<TypedOperand, TranslateError> {
        operand.map(|name, vector_index| self.replace_sreg(name, is_dst, vector_index))
    }

    fn visit_ident(
        &mut self,
        args: SpirvWord,
        _type_space: Option<(&ptx_parser::Type, ptx_parser::StateSpace)>,
        is_dst: bool,
        _relaxed_type_check: bool,
    ) -> Result<SpirvWord, TranslateError> {
        self.replace_sreg(args, is_dst, None)
    }
}

impl<'a, 'b, 'input> SpecialRegisterResolver<'a, 'b, 'input> {
    fn replace_sreg(
        &mut self,
        name: SpirvWord,
        is_dst: bool,
        vector_index: Option<u8>,
    ) -> Result<SpirvWord, TranslateError> {
        if let Some(sreg) = self.numeric_id_defs.special_registers.get(name) {
            if is_dst {
                return Err(TranslateError::MismatchedType);
            }
            let input_arguments = match (vector_index, sreg.get_function_input_type()) {
                (Some(idx), Some(inp_type)) => {
                    if inp_type != ast::ScalarType::U8 {
                        return Err(TranslateError::Unreachable);
                    }
                    let constant = self.numeric_id_defs.register_intermediate(Some((
                        ast::Type::Scalar(inp_type),
                        ast::StateSpace::Reg,
                    )));
                    self.result.push(Statement::Constant(ConstantDefinition {
                        dst: constant,
                        typ: inp_type,
                        value: ast::ImmediateValue::U64(idx as u64),
                    }));
                    vec![(
                        TypedOperand::Reg(constant),
                        ast::Type::Scalar(inp_type),
                        ast::StateSpace::Reg,
                    )]
                }
                (None, None) => Vec::new(),
                _ => return Err(TranslateError::MismatchedType),
            };
            let ocl_fn_name = [ZLUDA_PTX_PREFIX, sreg.get_unprefixed_function_name()].concat();
            let return_type = sreg.get_function_return_type();
            let fn_result = self.numeric_id_defs.register_intermediate(Some((
                ast::Type::Scalar(return_type),
                ast::StateSpace::Reg,
            )));
            let return_arguments = vec![(
                fn_result,
                ast::Type::Scalar(return_type),
                ast::StateSpace::Reg,
            )];
            let fn_call = register_external_fn_call(
                self.numeric_id_defs,
                self.ptx_impl_imports,
                ocl_fn_name.to_string(),
                return_arguments.iter().map(|(_, typ, space)| (typ, *space)),
                input_arguments.iter().map(|(_, typ, space)| (typ, *space)),
            )?;
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
            let arguments = ast::CallArgs {
                return_arguments: return_arguments.iter().map(|(name, _, _)| *name).collect(),
                func: fn_call,
                input_arguments: input_arguments.iter().map(|(name, _, _)| *name).collect(),
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
