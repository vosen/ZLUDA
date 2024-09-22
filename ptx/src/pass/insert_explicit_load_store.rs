use super::*;
use ptx_parser::VisitorMap;
use rustc_hash::FxHashSet;

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
    directives: Vec<Directive2<'input, ast::Instruction<SpirvWord>, SpirvWord>>,
) -> Result<Vec<Directive2<'input, ast::Instruction<SpirvWord>, SpirvWord>>, TranslateError> {
    directives
        .into_iter()
        .map(|directive| run_directive(resolver, directive))
        .collect::<Result<Vec<_>, _>>()
}

fn run_directive<'a, 'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    directive: Directive2<'input, ast::Instruction<SpirvWord>, SpirvWord>,
) -> Result<Directive2<'input, ast::Instruction<SpirvWord>, SpirvWord>, TranslateError> {
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
    method: Function2<'input, ast::Instruction<SpirvWord>, SpirvWord>,
) -> Result<Function2<'input, ast::Instruction<SpirvWord>, SpirvWord>, TranslateError> {
    let mut func_decl = method.func_decl;
    for arg in func_decl.return_arguments.iter_mut() {
        visitor.visit_variable(arg);
    }
    let is_kernel = func_decl.name.is_kernel();
    // let mut prelude = Vec::with_capacity(method.body.as_ref().map(Vec::len).unwrap_or(0));
    if is_kernel {
        for arg in func_decl.input_arguments.iter_mut() {
            let old_name = arg.name;
            let old_space = arg.state_space;
            let new_space = ast::StateSpace::ParamEntry;
            let new_name = visitor
                .resolver
                .register_unnamed(Some((arg.v_type.clone(), new_space)));
            visitor.input_argument(old_name, new_name, old_space);
            arg.name = new_name;
            arg.state_space = new_space;
        }
    };
    let body = method
        .body
        .map(move |statements| {
            let mut result = Vec::with_capacity(statements.len());
            for statement in statements {
                run_statement(&mut visitor, &mut result, statement)?;
            }
            Ok::<_, TranslateError>(result)
        })
        .transpose()?;
    Ok(Function2 {
        func_decl: func_decl,
        globals: method.globals,
        body,
        import_as: method.import_as,
        tuning: method.tuning,
        linkage: method.linkage,
    })
}

fn run_statement<'a, 'input>(
    visitor: &mut InsertMemSSAVisitor<'a, 'input>,
    result: &mut Vec<ExpandedStatement>,
    statement: ExpandedStatement,
) -> Result<(), TranslateError> {
    match statement {
        Statement::Variable(mut var) => {
            visitor.visit_variable(&mut var);
            result.push(Statement::Variable(var));
        }
        Statement::Instruction(ast::Instruction::Ld { data, arguments }) => {
            let instruction = visitor.visit_ld(data, arguments)?;
            let instruction = ast::visit_map(instruction, visitor)?;
            result.push(Statement::Instruction(instruction));
        }
        Statement::Instruction(ast::Instruction::St {
            data,
            mut arguments,
        }) => {
            let instruction = visitor.visit_st(data, arguments)?;
            let instruction = ast::visit_map(instruction, visitor)?;
            result.push(Statement::Instruction(instruction));
        }
        s => result.push(s.visit_map(visitor)?),
    }
    Ok(())
}

struct InsertMemSSAVisitor<'a, 'input> {
    resolver: &'a mut GlobalStringIdentResolver2<'input>,
    variables: FxHashMap<SpirvWord, RemapAction>,
}

impl<'a, 'input> InsertMemSSAVisitor<'a, 'input> {
    fn new(resolver: &'a mut GlobalStringIdentResolver2<'input>) -> Self {
        Self {
            resolver,
            variables: FxHashMap::default(),
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
        old_name: SpirvWord,
        new_name: SpirvWord,
        old_space: ast::StateSpace,
    ) -> Result<(), TranslateError> {
        Ok(match old_space {
            ast::StateSpace::Reg => {
                self.variables
                    .insert(old_name, RemapAction::PreLdPostSt(new_name));
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
            }
            // Good as-is
            ast::StateSpace::Local => {}
            // Will be pulled into global scope later
            ast::StateSpace::Generic
            | ast::StateSpace::SharedCluster
            | ast::StateSpace::Global
            | ast::StateSpace::Const
            | ast::StateSpace::SharedCta
            | ast::StateSpace::Shared => {}
            ast::StateSpace::ParamEntry | ast::StateSpace::ParamFunc => {
                return Err(error_unreachable())
            }
        })
    }

    fn visit_st(
        &self,
        mut data: ast::StData,
        mut arguments: ast::StArgs<SpirvWord>,
    ) -> Result<ast::Instruction<SpirvWord>, TranslateError> {
        if let Some(remap) = self.variables.get(&arguments.src1) {
            match remap {
                RemapAction::PreLdPostSt(_) => return Err(error_mismatched_type()),
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
                RemapAction::PreLdPostSt(_) => return Err(error_mismatched_type()),
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

    fn visit_variable(&mut self, var: &mut ast::Variable<SpirvWord>) {
        if var.state_space != ast::StateSpace::Local {
            let old_name = var.name;
            let old_space = var.state_space;
            let new_space = ast::StateSpace::Local;
            let new_name = self
                .resolver
                .register_unnamed(Some((var.v_type.clone(), new_space)));
            self.variable(old_name, new_name, old_space);
            var.name = new_name;
            var.state_space = new_space;
        }
    }
}

impl<'a, 'input> ast::VisitorMap<SpirvWord, SpirvWord, TranslateError>
    for InsertMemSSAVisitor<'a, 'input>
{
    fn visit(
        &mut self,
        args: SpirvWord,
        type_space: Option<(&ast::Type, ast::StateSpace)>,
        is_dst: bool,
        relaxed_type_check: bool,
    ) -> Result<SpirvWord, TranslateError> {
        todo!()
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

#[derive(Clone, Copy)]
enum RemapAction {
    PreLdPostSt(SpirvWord),
    LDStSpaceChange {
        old_space: ast::StateSpace,
        new_space: ast::StateSpace,
        name: SpirvWord,
    },
}
