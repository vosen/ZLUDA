use super::*;
// This pass:
// * Turns all .local, .param and .reg in-body variables into .local variables
//   (if _not_ an input method argument)
// * Inserts explicit `ld`/`st` for newly converted .reg variables
// * Fixup state space of all existing `ld`/`st` instructions into newly
//   converted variables
// * Turns `.entry` input arguments into param::entry and all related `.param`
//   loads into `param::entry` loads
// * All `.func` input arguments are turned into `.reg` arguments by another
//   pass, so we do nothing there
pub(super) fn run<'a, 'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    directives: Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>>,
) -> Result<Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>>, TranslateError> {
    directives
        .into_iter()
        .map(|directive| run_directive(resolver, directive))
        .collect::<Result<Vec<_>, _>>()
}

fn run_directive<'a, 'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    directive: Directive2<ast::Instruction<SpirvWord>, SpirvWord>,
) -> Result<Directive2<ast::Instruction<SpirvWord>, SpirvWord>, TranslateError> {
    Ok(match directive {
        var @ Directive2::Variable(..) => var,
        Directive2::Method(method) => {
            let visitor = InsertMemSSAVisitor::new(resolver);
            Directive2::Method(run_method(visitor, method)?)
        }
    })
}

fn run_method<'a, 'input>(
    mut visitor: InsertMemSSAVisitor<'a, 'input>,
    mut method: Function2<ast::Instruction<SpirvWord>, SpirvWord>,
) -> Result<Function2<ast::Instruction<SpirvWord>, SpirvWord>, TranslateError> {
    if method.is_kernel() {
        for arg in method.input_arguments.iter_mut() {
            let old_name = arg.name;
            let old_space = arg.info.state_space;
            let new_space = ast::StateSpace::ParamEntry;
            let new_name = visitor
                .resolver
                .register_unnamed(Some((arg.info.v_type.clone(), new_space)));
            visitor.input_argument(old_name, new_name, old_space)?;
            arg.name = new_name;
            arg.info.state_space = new_space;
        }
    };
    for arg in method.return_arguments.iter_mut() {
        visitor.visit_variable(arg)?;
    }
    let return_arguments = &method.return_arguments[..];
    let body = method
        .body
        .map(move |statements| {
            let mut result = Vec::with_capacity(statements.len());
            for statement in statements {
                run_statement(&mut visitor, return_arguments, &mut result, statement)?;
            }
            Ok::<_, TranslateError>(result)
        })
        .transpose()?;
    Ok(Function2 { body, ..method })
}

fn run_statement<'a, 'input>(
    visitor: &mut InsertMemSSAVisitor<'a, 'input>,
    return_arguments: &[ast::Variable<SpirvWord>],
    result: &mut Vec<ExpandedStatement>,
    statement: ExpandedStatement,
) -> Result<(), TranslateError> {
    match statement {
        Statement::Instruction(ast::Instruction::Ret { data }) => {
            let statement = if return_arguments.is_empty() {
                Statement::Instruction(ast::Instruction::Ret { data })
            } else {
                Statement::RetValue(
                    data,
                    return_arguments
                        .iter()
                        .map(|arg| {
                            if arg.info.state_space != ast::StateSpace::Local {
                                return Err(error_unreachable());
                            }
                            Ok((arg.name, arg.info.v_type.clone()))
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                )
            };
            let new_statement = statement.visit_map(visitor)?;
            result.extend(visitor.pre.drain(..).map(Statement::Instruction));
            result.push(new_statement);
            result.extend(visitor.post.drain(..).map(Statement::Instruction));
        }
        Statement::Variable(mut var) => {
            visitor.visit_variable(&mut var)?;
            result.push(Statement::Variable(var));
        }
        Statement::Instruction(ast::Instruction::Ld { data, arguments }) => {
            let instruction = visitor.visit_ld(data, arguments)?;
            let instruction = ast::visit_map(instruction, visitor)?;
            result.extend(visitor.pre.drain(..).map(Statement::Instruction));
            result.push(Statement::Instruction(instruction));
            result.extend(visitor.post.drain(..).map(Statement::Instruction));
        }
        Statement::Instruction(ast::Instruction::St { data, arguments }) => {
            let instruction = visitor.visit_st(data, arguments)?;
            let instruction = ast::visit_map(instruction, visitor)?;
            result.extend(visitor.pre.drain(..).map(Statement::Instruction));
            result.push(Statement::Instruction(instruction));
            result.extend(visitor.post.drain(..).map(Statement::Instruction));
        }
        Statement::Instruction(ast::Instruction::Mov { data, arguments }) => {
            let instruction = visitor.visit_mov(data, arguments);
            let instruction = ast::visit_map(instruction, visitor)?;
            result.extend(visitor.pre.drain(..).map(Statement::Instruction));
            result.push(Statement::Instruction(instruction));
            result.extend(visitor.post.drain(..).map(Statement::Instruction));
        }
        Statement::PtrAccess(ptr_access) => {
            let statement = Statement::PtrAccess(visitor.visit_ptr_access(ptr_access)?);
            let statement = statement.visit_map(visitor)?;
            result.extend(visitor.pre.drain(..).map(Statement::Instruction));
            result.push(statement);
            result.extend(visitor.post.drain(..).map(Statement::Instruction));
        }
        s => {
            let new_statement = s.visit_map(visitor)?;
            result.extend(visitor.pre.drain(..).map(Statement::Instruction));
            result.push(new_statement);
            result.extend(visitor.post.drain(..).map(Statement::Instruction));
        }
    }
    Ok(())
}

struct InsertMemSSAVisitor<'a, 'input> {
    resolver: &'a mut GlobalStringIdentResolver2<'input>,
    variables: FxHashMap<SpirvWord, RemapAction>,
    pre: Vec<ast::Instruction<SpirvWord>>,
    post: Vec<ast::Instruction<SpirvWord>>,
}

impl<'a, 'input> InsertMemSSAVisitor<'a, 'input> {
    fn new(resolver: &'a mut GlobalStringIdentResolver2<'input>) -> Self {
        Self {
            resolver,
            variables: FxHashMap::default(),
            pre: Vec::new(),
            post: Vec::new(),
        }
    }

    fn input_argument(
        &mut self,
        old_name: SpirvWord,
        new_name: SpirvWord,
        old_space: ast::StateSpace,
    ) -> Result<(), TranslateError> {
        if old_space != ast::StateSpace::Param {
            return Err(error_unreachable());
        }
        self.variables.insert(
            old_name,
            RemapAction::LDStSpaceChange {
                name: new_name,
                old_space,
                new_space: ast::StateSpace::ParamEntry,
            },
        );
        Ok(())
    }

    fn variable(
        &mut self,
        type_: &ast::Type,
        old_name: SpirvWord,
        new_name: SpirvWord,
        old_space: ast::StateSpace,
    ) -> Result<bool, TranslateError> {
        Ok(match old_space {
            ast::StateSpace::Reg => {
                self.variables.insert(
                    old_name,
                    RemapAction::PreLdPostSt {
                        name: new_name,
                        type_: type_.clone(),
                    },
                );
                true
            }
            ast::StateSpace::Param => {
                self.variables.insert(
                    old_name,
                    RemapAction::LDStSpaceChange {
                        old_space,
                        new_space: ast::StateSpace::Local,
                        name: new_name,
                    },
                );
                true
            }
            // Good as-is
            ast::StateSpace::Local
            | ast::StateSpace::Generic
            | ast::StateSpace::SharedCluster
            | ast::StateSpace::Global
            | ast::StateSpace::Const
            | ast::StateSpace::SharedCta
            | ast::StateSpace::Shared
            | ast::StateSpace::ParamEntry
            | ast::StateSpace::ParamFunc => return Err(error_unreachable()),
        })
    }

    fn visit_st(
        &self,
        mut data: ast::StData,
        mut arguments: ast::StArgs<SpirvWord>,
    ) -> Result<ast::Instruction<SpirvWord>, TranslateError> {
        if let Some(remap) = self.variables.get(&arguments.src1) {
            match remap {
                RemapAction::PreLdPostSt { .. } => {}
                RemapAction::LDStSpaceChange {
                    old_space,
                    new_space,
                    name,
                } => {
                    if data.state_space != *old_space {
                        return Err(error_mismatched_type());
                    }
                    data.state_space = *new_space;
                    arguments.src1 = *name;
                }
            }
        }
        Ok(ast::Instruction::St { data, arguments })
    }

    fn visit_ld(
        &self,
        mut data: ast::LdDetails,
        mut arguments: ast::LdArgs<SpirvWord>,
    ) -> Result<ast::Instruction<SpirvWord>, TranslateError> {
        if let Some(remap) = self.variables.get(&arguments.src) {
            match remap {
                RemapAction::PreLdPostSt { .. } => {}
                RemapAction::LDStSpaceChange {
                    old_space,
                    new_space,
                    name,
                } => {
                    if data.state_space != *old_space {
                        return Err(error_mismatched_type());
                    }
                    data.state_space = *new_space;
                    arguments.src = *name;
                }
            }
        }
        Ok(ast::Instruction::Ld { data, arguments })
    }

    fn visit_ptr_access(
        &mut self,
        ptr_access: PtrAccess<SpirvWord>,
    ) -> Result<PtrAccess<SpirvWord>, TranslateError> {
        let (old_space, new_space, name) = match self.variables.get(&ptr_access.ptr_src) {
            Some(RemapAction::LDStSpaceChange {
                old_space,
                new_space,
                name,
            }) => (*old_space, *new_space, *name),
            Some(RemapAction::PreLdPostSt { .. }) | None => return Ok(ptr_access),
        };
        if ptr_access.state_space != old_space {
            return Err(error_mismatched_type());
        }
        // Propagate space changes in dst
        let new_dst = self
            .resolver
            .register_unnamed(Some((ptr_access.underlying_type.clone(), new_space)));
        self.variables.insert(
            ptr_access.dst,
            RemapAction::LDStSpaceChange {
                old_space,
                new_space,
                name: new_dst,
            },
        );
        Ok(PtrAccess {
            ptr_src: name,
            dst: new_dst,
            state_space: new_space,
            ..ptr_access
        })
    }

    fn visit_mov(
        &mut self,
        data: ptx_parser::MovDetails,
        mut arguments: ptx_parser::MovArgs<SpirvWord>,
    ) -> ast::Instruction<SpirvWord> {
        if let Some(remap) = self.variables.get(&arguments.src) {
            match remap {
                RemapAction::PreLdPostSt { .. } => {}
                RemapAction::LDStSpaceChange {
                    name, new_space, ..
                } => {
                    let generic_var = self
                        .resolver
                        .register_unnamed(Some((data.typ.clone(), ast::StateSpace::Reg)));
                    self.pre.push(ast::Instruction::Cvta {
                        data: ast::CvtaDetails {
                            state_space: *new_space,
                            direction: ast::CvtaDirection::ExplicitToGeneric,
                        },
                        arguments: ast::CvtaArgs {
                            dst: generic_var,
                            src: *name,
                        },
                    });
                    arguments.src = generic_var;
                }
            }
        }
        ast::Instruction::Mov { data, arguments }
    }

    fn visit_variable(&mut self, var: &mut ast::Variable<SpirvWord>) -> Result<(), TranslateError> {
        let old_space = match var.info.state_space {
            space @ (ptx_parser::StateSpace::Reg | ptx_parser::StateSpace::Param) => space,
            // Do nothing
            ptx_parser::StateSpace::Local => return Ok(()),
            // Handled by another pass
            ptx_parser::StateSpace::Generic
            | ptx_parser::StateSpace::SharedCluster
            | ptx_parser::StateSpace::ParamEntry
            | ptx_parser::StateSpace::Global
            | ptx_parser::StateSpace::SharedCta
            | ptx_parser::StateSpace::Const
            | ptx_parser::StateSpace::Shared
            | ptx_parser::StateSpace::ParamFunc => return Ok(()),
        };
        let old_name = var.name;
        let new_space = ast::StateSpace::Local;
        let new_name = self
            .resolver
            .register_unnamed(Some((var.info.v_type.clone(), new_space)));
        self.variable(&var.info.v_type, old_name, new_name, old_space)?;
        var.name = new_name;
        var.info.state_space = new_space;
        Ok(())
    }
}

impl<'a, 'input> ast::VisitorMap<SpirvWord, SpirvWord, TranslateError>
    for InsertMemSSAVisitor<'a, 'input>
{
    fn visit(
        &mut self,
        ident: SpirvWord,
        _type_space: Option<(&ast::Type, ast::StateSpace)>,
        is_dst: bool,
        _relaxed_type_check: bool,
    ) -> Result<SpirvWord, TranslateError> {
        if let Some(remap) = self.variables.get(&ident) {
            match remap {
                RemapAction::PreLdPostSt { name, type_ } => {
                    if is_dst {
                        let temp = self
                            .resolver
                            .register_unnamed(Some((type_.clone(), ast::StateSpace::Reg)));
                        self.post.push(ast::Instruction::St {
                            data: ast::StData {
                                state_space: ast::StateSpace::Local,
                                qualifier: ast::LdStQualifier::Weak,
                                caching: ast::StCacheOperator::Writethrough,
                                typ: type_.clone(),
                            },
                            arguments: ast::StArgs {
                                src1: *name,
                                src2: temp,
                            },
                        });
                        Ok(temp)
                    } else {
                        let temp = self
                            .resolver
                            .register_unnamed(Some((type_.clone(), ast::StateSpace::Reg)));
                        self.pre.push(ast::Instruction::Ld {
                            data: ast::LdDetails {
                                state_space: ast::StateSpace::Local,
                                qualifier: ast::LdStQualifier::Weak,
                                caching: ast::LdCacheOperator::Cached,
                                typ: type_.clone(),
                                non_coherent: false,
                            },
                            arguments: ast::LdArgs {
                                dst: temp,
                                src: *name,
                            },
                        });
                        Ok(temp)
                    }
                }
                RemapAction::LDStSpaceChange { .. } => {
                    return Err(error_mismatched_type());
                }
            }
        } else {
            Ok(ident)
        }
    }

    fn visit_ident(
        &mut self,
        args: SpirvWord,
        type_space: Option<(&ast::Type, ast::StateSpace)>,
        is_dst: bool,
        relaxed_type_check: bool,
    ) -> Result<SpirvWord, TranslateError> {
        self.visit(args, type_space, is_dst, relaxed_type_check)
    }
}

#[derive(Clone)]
enum RemapAction {
    PreLdPostSt {
        name: SpirvWord,
        type_: ast::Type,
    },
    LDStSpaceChange {
        old_space: ast::StateSpace,
        new_space: ast::StateSpace,
        name: SpirvWord,
    },
}
