use std::collections::HashMap;
use half::f16;
use ptx_parser as ast;

fn to_ssa<'input, 'b>(
    ptx_impl_imports: &'b mut HashMap<String, Directive<'input>>,
    mut id_defs: FnStringIdResolver<'input, 'b>,
    fn_defs: GlobalFnDeclResolver<'input, 'b>,
    func_decl: Rc<RefCell<ast::MethodDeclaration<'input, spirv::Word>>>,
    f_body: Option<Vec<ast::Statement<ast::ParsedArgParams<'input>>>>,
    tuning: Vec<ast::TuningDirective>,
    linkage: ast::LinkingDirective,
) -> Result<Function<'input>, TranslateError> {
    //deparamize_function_decl(&func_decl)?;
    let f_body = match f_body {
        Some(vec) => vec,
        None => {
            return Ok(Function {
                func_decl: func_decl,
                body: None,
                globals: Vec::new(),
                import_as: None,
                tuning,
                linkage,
            })
        }
    };
    let normalized_ids = normalize_identifiers(&mut id_defs, &fn_defs, f_body)?;
    /*
    let mut numeric_id_defs = id_defs.finish();
    let unadorned_statements = normalize_predicates(normalized_ids, &mut numeric_id_defs)?;
    let typed_statements =
        convert_to_typed_statements(unadorned_statements, &fn_defs, &mut numeric_id_defs)?;
    let typed_statements =
        fix_special_registers2(ptx_impl_imports, typed_statements, &mut numeric_id_defs)?;
    let (func_decl, typed_statements) =
        convert_to_stateful_memory_access(func_decl, typed_statements, &mut numeric_id_defs)?;
    let ssa_statements = insert_mem_ssa_statements(
        typed_statements,
        &mut numeric_id_defs,
        &mut (*func_decl).borrow_mut(),
    )?;
    let mut numeric_id_defs = numeric_id_defs.finish();
    let expanded_statements = expand_arguments(ssa_statements, &mut numeric_id_defs)?;
    let expanded_statements =
        insert_implicit_conversions(expanded_statements, &mut numeric_id_defs)?;
    let mut numeric_id_defs = numeric_id_defs.unmut();
    let labeled_statements = normalize_labels(expanded_statements, &mut numeric_id_defs);
    let (f_body, globals) =
        extract_globals(labeled_statements, ptx_impl_imports, &mut numeric_id_defs)?;
    Ok(Function {
        func_decl: func_decl,
        globals: globals,
        body: Some(f_body),
        import_as: None,
        tuning,
        linkage,
    })
     */
}