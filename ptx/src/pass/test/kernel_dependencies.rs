use super::super::kernel_dependencies::{
    build_compilation_plan, direct_callees, function_index, global_declaration,
    kernel_declaration_sets, kernel_dependencies, kernel_method_sets, method_declaration,
    reachable_callees,
};
use super::super::*;

#[test]
fn extracts_direct_callee() {
    let callee = SpirvWord(2);

    let function = Function {
        return_arguments: Vec::new(),
        name: SpirvWord(1),
        input_arguments: Vec::new(),
        body: Some(vec![Statement::Instruction(ast::Instruction::Call {
            data: ast::CallDetails {
                uniform: false,
                return_arguments: Vec::new(),
                input_arguments: Vec::new(),
            },
            arguments: ast::CallArgs {
                return_arguments: Vec::new(),
                func: callee,
                input_arguments: Vec::new(),
                is_external: false,
            },
        })]),
        kernel_attributes: None,
        import_as: None,
        tuning: Vec::new(),
        linkage: ast::LinkingDirective::NONE,
        kernel_meta32: None,
    };

    let callees = direct_callees(&function);

    assert_eq!(callees.len(), 1);
    assert!(callees.contains(&callee));
}

#[test]
fn declaration_has_no_callees() {
    let function = Function {
        return_arguments: Vec::new(),
        name: SpirvWord(1),
        input_arguments: Vec::new(),
        body: None,
        kernel_attributes: None,
        import_as: None,
        tuning: Vec::new(),
        linkage: ast::LinkingDirective::NONE,
        kernel_meta32: None,
    };

    assert!(direct_callees(&function).is_empty());
}

#[test]
fn finds_transitive_callees_from_function_index() {
    let kernel_name = SpirvWord(1);
    let helper_a_name = SpirvWord(2);
    let helper_b_name = SpirvWord(3);

    let make_call = |callee| {
        Statement::Instruction(ast::Instruction::Call {
            data: ast::CallDetails {
                uniform: false,
                return_arguments: Vec::new(),
                input_arguments: Vec::new(),
            },
            arguments: ast::CallArgs {
                return_arguments: Vec::new(),
                func: callee,
                input_arguments: Vec::new(),
                is_external: false,
            },
        })
    };

    let make_function = |name, body| Function {
        return_arguments: Vec::new(),
        name,
        input_arguments: Vec::new(),
        body,
        kernel_attributes: None,
        import_as: None,
        tuning: Vec::new(),
        linkage: ast::LinkingDirective::NONE,
        kernel_meta32: None,
    };

    let directives = vec![
        Directive2::Method(make_function(
            kernel_name,
            Some(vec![make_call(helper_a_name)]),
        )),
        Directive2::Method(make_function(
            helper_a_name,
            Some(vec![make_call(helper_b_name)]),
        )),
        Directive2::Method(make_function(helper_b_name, Some(Vec::new()))),
    ];

    let functions = function_index(&directives);
    let reachable = reachable_callees(kernel_name, &functions);

    assert_eq!(reachable.len(), 2);
    assert!(reachable.contains(&helper_a_name));
    assert!(reachable.contains(&helper_b_name));
    assert!(!reachable.contains(&kernel_name));
}

#[test]
fn excludes_root_when_call_graph_contains_cycle() {
    let root_name = SpirvWord(1);
    let helper_name = SpirvWord(2);

    let make_call = |callee| {
        Statement::Instruction(ast::Instruction::Call {
            data: ast::CallDetails {
                uniform: false,
                return_arguments: Vec::new(),
                input_arguments: Vec::new(),
            },
            arguments: ast::CallArgs {
                return_arguments: Vec::new(),
                func: callee,
                input_arguments: Vec::new(),
                is_external: false,
            },
        })
    };

    let make_function = |name, callee| Function {
        return_arguments: Vec::new(),
        name,
        input_arguments: Vec::new(),
        body: Some(vec![make_call(callee)]),
        kernel_attributes: None,
        import_as: None,
        tuning: Vec::new(),
        linkage: ast::LinkingDirective::NONE,
        kernel_meta32: None,
    };

    let directives = vec![
        Directive2::Method(make_function(root_name, helper_name)),
        Directive2::Method(make_function(helper_name, root_name)),
    ];

    let functions = function_index(&directives);
    let reachable = reachable_callees(root_name, &functions);

    assert_eq!(reachable.len(), 1);
    assert!(reachable.contains(&helper_name));
    assert!(!reachable.contains(&root_name));
}

#[test]
fn computes_dependencies_for_each_kernel() {
    let kernel_name = SpirvWord(1);
    let helper_a_name = SpirvWord(2);
    let helper_b_name = SpirvWord(3);

    let make_call = |callee| {
        Statement::Instruction(ast::Instruction::Call {
            data: ast::CallDetails {
                uniform: false,
                return_arguments: Vec::new(),
                input_arguments: Vec::new(),
            },
            arguments: ast::CallArgs {
                return_arguments: Vec::new(),
                func: callee,
                input_arguments: Vec::new(),
                is_external: false,
            },
        })
    };

    let make_function = |name, body, kernel_attributes| Function {
        return_arguments: Vec::new(),
        name,
        input_arguments: Vec::new(),
        body,
        kernel_attributes,
        import_as: None,
        tuning: Vec::new(),
        linkage: ast::LinkingDirective::NONE,
        kernel_meta32: None,
    };

    let kernel_attributes = KernelAttributes {
        flush_to_zero_f32: false,
        flush_to_zero_f16f64: false,
        rounding_mode_f32: ast::RoundingMode::NearestEven,
        rounding_mode_f16f64: ast::RoundingMode::NearestEven,
    };

    let directives = vec![
        Directive2::Method(make_function(
            kernel_name,
            Some(vec![make_call(helper_a_name)]),
            Some(kernel_attributes),
        )),
        Directive2::Method(make_function(
            helper_a_name,
            Some(vec![make_call(helper_b_name)]),
            None,
        )),
        Directive2::Method(make_function(helper_b_name, Some(Vec::new()), None)),
    ];

    let dependencies = kernel_dependencies(&directives);
    let kernel_callees = dependencies
        .get(&kernel_name)
        .expect("kernel dependency entry should exist");

    assert_eq!(dependencies.len(), 1);
    assert_eq!(kernel_callees.len(), 2);
    assert!(kernel_callees.contains(&helper_a_name));
    assert!(kernel_callees.contains(&helper_b_name));
    assert!(!kernel_callees.contains(&kernel_name));
}

#[test]
fn includes_kernel_in_its_method_set() {
    let kernel_name = SpirvWord(1);
    let helper_a_name = SpirvWord(2);
    let helper_b_name = SpirvWord(3);

    let make_call = |callee| {
        Statement::Instruction(ast::Instruction::Call {
            data: ast::CallDetails {
                uniform: false,
                return_arguments: Vec::new(),
                input_arguments: Vec::new(),
            },
            arguments: ast::CallArgs {
                return_arguments: Vec::new(),
                func: callee,
                input_arguments: Vec::new(),
                is_external: false,
            },
        })
    };

    let make_function = |name, body, kernel_attributes| Function {
        return_arguments: Vec::new(),
        name,
        input_arguments: Vec::new(),
        body,
        kernel_attributes,
        import_as: None,
        tuning: Vec::new(),
        linkage: ast::LinkingDirective::NONE,
        kernel_meta32: None,
    };

    let kernel_attributes = KernelAttributes {
        flush_to_zero_f32: false,
        flush_to_zero_f16f64: false,
        rounding_mode_f32: ast::RoundingMode::NearestEven,
        rounding_mode_f16f64: ast::RoundingMode::NearestEven,
    };

    let directives = vec![
        Directive2::Method(make_function(
            kernel_name,
            Some(vec![make_call(helper_a_name)]),
            Some(kernel_attributes),
        )),
        Directive2::Method(make_function(
            helper_a_name,
            Some(vec![make_call(helper_b_name)]),
            None,
        )),
        Directive2::Method(make_function(helper_b_name, Some(Vec::new()), None)),
    ];

    let method_sets = kernel_method_sets(&directives);
    let methods = method_sets
        .get(&kernel_name)
        .expect("kernel method set should exist");

    assert_eq!(method_sets.len(), 1);
    assert_eq!(methods.len(), 3);
    assert!(methods.contains(&kernel_name));
    assert!(methods.contains(&helper_a_name));
    assert!(methods.contains(&helper_b_name));
}

#[test]
fn creates_bodyless_helper_declaration() {
    let helper_name = SpirvWord(10);
    let input_name = SpirvWord(11);
    let return_name = SpirvWord(12);

    let helper = Function {
        return_arguments: vec![ast::Variable {
            name: return_name,
            info: ast::VariableInfo {
                align: None,
                v_type: ast::Type::Scalar(ast::ScalarType::U32),
                state_space: ast::StateSpace::Param,
                array_init: Vec::new(),
            },
        }],
        name: helper_name,
        input_arguments: vec![ast::Variable {
            name: input_name,
            info: ast::VariableInfo {
                align: None,
                v_type: ast::Type::Scalar(ast::ScalarType::U32),
                state_space: ast::StateSpace::Param,
                array_init: Vec::new(),
            },
        }],
        body: Some(Vec::new()),
        kernel_attributes: None,
        import_as: None,
        tuning: vec![ast::TuningDirective::NoReturn],
        linkage: ast::LinkingDirective::VISIBLE,
        kernel_meta32: None,
    };

    let declaration = method_declaration(&helper);

    assert_eq!(declaration.name, helper_name);
    assert_eq!(declaration.return_arguments.len(), 1);
    assert_eq!(declaration.input_arguments.len(), 1);
    assert!(declaration.body.is_none());
    assert!(!declaration.is_kernel());

    assert!(helper.body.is_some());
}

#[test]
fn builds_helper_declarations_for_kernel() {
    let kernel_name = SpirvWord(1);
    let helper_name = SpirvWord(2);

    let make_call = |callee| {
        Statement::Instruction(ast::Instruction::Call {
            data: ast::CallDetails {
                uniform: false,
                return_arguments: Vec::new(),
                input_arguments: Vec::new(),
            },
            arguments: ast::CallArgs {
                return_arguments: Vec::new(),
                func: callee,
                input_arguments: Vec::new(),
                is_external: false,
            },
        })
    };

    let make_function = |name, body, kernel_attributes| Function {
        return_arguments: Vec::new(),
        name,
        input_arguments: Vec::new(),
        body,
        kernel_attributes,
        import_as: None,
        tuning: Vec::new(),
        linkage: ast::LinkingDirective::NONE,
        kernel_meta32: None,
    };

    let kernel_attributes = KernelAttributes {
        flush_to_zero_f32: false,
        flush_to_zero_f16f64: false,
        rounding_mode_f32: ast::RoundingMode::NearestEven,
        rounding_mode_f16f64: ast::RoundingMode::NearestEven,
    };

    let directives = vec![
        Directive2::Method(make_function(
            kernel_name,
            Some(vec![make_call(helper_name)]),
            Some(kernel_attributes),
        )),
        Directive2::Method(make_function(helper_name, Some(Vec::new()), None)),
    ];

    let declaration_sets = kernel_declaration_sets(&directives);
    let declarations = declaration_sets
        .get(&kernel_name)
        .expect("kernel declaration set should exist");

    assert_eq!(declaration_sets.len(), 1);
    assert_eq!(declarations.len(), 1);
    assert_eq!(declarations[0].name, helper_name);
    assert!(declarations[0].body.is_none());
    assert!(!declarations[0].is_kernel());
}

#[test]
fn compilation_plan_separates_kernels_from_common_directives() {
    let global_name = SpirvWord(1);
    let kernel_name = SpirvWord(2);
    let helper_name = SpirvWord(3);

    let make_call = |callee| {
        Statement::Instruction(ast::Instruction::Call {
            data: ast::CallDetails {
                uniform: false,
                return_arguments: Vec::new(),
                input_arguments: Vec::new(),
            },
            arguments: ast::CallArgs {
                return_arguments: Vec::new(),
                func: callee,
                input_arguments: Vec::new(),
                is_external: false,
            },
        })
    };

    let make_function = |name, body, kernel_attributes| Function {
        return_arguments: Vec::new(),
        name,
        input_arguments: Vec::new(),
        body,
        kernel_attributes,
        import_as: None,
        tuning: Vec::new(),
        linkage: ast::LinkingDirective::NONE,
        kernel_meta32: None,
    };

    let kernel_attributes = KernelAttributes {
        flush_to_zero_f32: false,
        flush_to_zero_f16f64: false,
        rounding_mode_f32: ast::RoundingMode::NearestEven,
        rounding_mode_f16f64: ast::RoundingMode::NearestEven,
    };

    let global = ast::Variable {
        name: global_name,
        info: ast::VariableInfo {
            align: None,
            v_type: ast::Type::Scalar(ast::ScalarType::U32),
            state_space: ast::StateSpace::Global,
            array_init: Vec::new(),
        },
    };

    let directives = vec![
        Directive2::Variable(ast::LinkingDirective::NONE, global),
        Directive2::Method(make_function(
            kernel_name,
            Some(vec![make_call(helper_name)]),
            Some(kernel_attributes),
        )),
        Directive2::Method(make_function(helper_name, Some(Vec::new()), None)),
    ];

    let plan = build_compilation_plan(directives);

    assert_eq!(plan.common.len(), 2);
    assert_eq!(plan.kernels.len(), 1);

    assert!(matches!(
        &plan.common[0],
        Directive2::Variable(_, variable) if variable.name == global_name
    ));
    assert!(matches!(
        &plan.common[1],
        Directive2::Method(function) if function.name == helper_name
    ));

    let kernel_plan = &plan.kernels[0];

    assert_eq!(kernel_plan.kernel.name, kernel_name);
    assert!(kernel_plan.kernel.is_kernel());
    assert_eq!(kernel_plan.declarations.len(), 1);
    assert_eq!(kernel_plan.declarations[0].name, helper_name);
    assert!(kernel_plan.declarations[0].body.is_none());
    assert!(!kernel_plan.declarations[0].is_kernel());
}

#[test]
fn compilation_plan_includes_only_reachable_helper_declarations() {
    let kernel_name = SpirvWord(1);
    let reachable_helper = SpirvWord(2);
    let unreachable_helper = SpirvWord(3);

    let make_call = |callee| {
        Statement::Instruction(ast::Instruction::Call {
            data: ast::CallDetails {
                uniform: false,
                return_arguments: Vec::new(),
                input_arguments: Vec::new(),
            },
            arguments: ast::CallArgs {
                return_arguments: Vec::new(),
                func: callee,
                input_arguments: Vec::new(),
                is_external: false,
            },
        })
    };

    let make_function = |name, body, kernel_attributes| Function {
        return_arguments: Vec::new(),
        name,
        input_arguments: Vec::new(),
        body,
        kernel_attributes,
        import_as: None,
        tuning: Vec::new(),
        linkage: ast::LinkingDirective::NONE,
        kernel_meta32: None,
    };

    let kernel_attributes = KernelAttributes {
        flush_to_zero_f32: false,
        flush_to_zero_f16f64: false,
        rounding_mode_f32: ast::RoundingMode::NearestEven,
        rounding_mode_f16f64: ast::RoundingMode::NearestEven,
    };

    let directives = vec![
        Directive2::Method(make_function(
            kernel_name,
            Some(vec![make_call(reachable_helper)]),
            Some(kernel_attributes),
        )),
        Directive2::Method(make_function(reachable_helper, Some(Vec::new()), None)),
        Directive2::Method(make_function(unreachable_helper, Some(Vec::new()), None)),
    ];

    let plan = build_compilation_plan(directives);
    let kernel_plan = &plan.kernels[0];

    assert_eq!(kernel_plan.declarations.len(), 1);
    assert_eq!(kernel_plan.declarations[0].name, reachable_helper);

    assert!(plan.common.iter().any(|directive| matches!(
        directive,
        Directive2::Method(function) if function.name == reachable_helper
    )));
    assert!(plan.common.iter().any(|directive| matches!(
        directive,
        Directive2::Method(function) if function.name == unreachable_helper
    )));
}

#[test]
fn monolithic_conversion_restores_definitions_without_declarations() {
    let kernel_name = SpirvWord(1);
    let helper_name = SpirvWord(2);

    let make_call = |callee| {
        Statement::Instruction(ast::Instruction::Call {
            data: ast::CallDetails {
                uniform: false,
                return_arguments: Vec::new(),
                input_arguments: Vec::new(),
            },
            arguments: ast::CallArgs {
                return_arguments: Vec::new(),
                func: callee,
                input_arguments: Vec::new(),
                is_external: false,
            },
        })
    };

    let make_function = |name, body, kernel_attributes| Function {
        return_arguments: Vec::new(),
        name,
        input_arguments: Vec::new(),
        body,
        kernel_attributes,
        import_as: None,
        tuning: Vec::new(),
        linkage: ast::LinkingDirective::NONE,
        kernel_meta32: None,
    };

    let kernel_attributes = KernelAttributes {
        flush_to_zero_f32: false,
        flush_to_zero_f16f64: false,
        rounding_mode_f32: ast::RoundingMode::NearestEven,
        rounding_mode_f16f64: ast::RoundingMode::NearestEven,
    };

    let directives = vec![
        Directive2::Method(make_function(
            kernel_name,
            Some(vec![make_call(helper_name)]),
            Some(kernel_attributes),
        )),
        Directive2::Method(make_function(helper_name, Some(Vec::new()), None)),
    ];

    let restored = build_compilation_plan(directives).into_monolithic_directives();

    assert_eq!(restored.len(), 2);

    assert!(restored.iter().any(|directive| matches!(
        directive,
        Directive2::Method(function)
            if function.name == helper_name && function.body.is_some()
    )));

    assert!(restored.iter().any(|directive| matches!(
        directive,
        Directive2::Method(function)
            if function.name == kernel_name && function.is_kernel()
    )));

    assert!(!restored.iter().any(|directive| matches!(
        directive,
        Directive2::Method(function)
            if function.name == helper_name && function.body.is_none()
    )));
}

#[test]
fn global_declaration_removes_initializer() {
    let global_name = SpirvWord(1);
    let variable = ast::Variable {
        name: global_name,
        info: ast::VariableInfo {
            align: Some(8),
            v_type: ast::Type::Scalar(ast::ScalarType::U32),
            state_space: ast::StateSpace::Global,
            array_init: vec![ast::RegOrImmediate::Imm(ast::ImmediateValue::U64(42))],
        },
    };

    let (linking, declaration) = global_declaration(ast::LinkingDirective::VISIBLE, &variable);

    assert!(linking.contains(ast::LinkingDirective::VISIBLE));
    assert_eq!(declaration.name, global_name);
    assert_eq!(declaration.info.align, Some(8));
    assert!(declaration.info.array_init.is_empty());

    assert_eq!(variable.info.array_init.len(), 1);
}

#[test]
fn compilation_plan_adds_global_declarations_to_each_kernel() {
    let global_name = SpirvWord(1);
    let first_kernel_name = SpirvWord(2);
    let second_kernel_name = SpirvWord(3);

    let make_kernel = |name| Function {
        return_arguments: Vec::new(),
        name,
        input_arguments: Vec::new(),
        body: Some(Vec::new()),
        kernel_attributes: Some(KernelAttributes {
            flush_to_zero_f32: false,
            flush_to_zero_f16f64: false,
            rounding_mode_f32: ast::RoundingMode::NearestEven,
            rounding_mode_f16f64: ast::RoundingMode::NearestEven,
        }),
        import_as: None,
        tuning: Vec::new(),
        linkage: ast::LinkingDirective::NONE,
        kernel_meta32: None,
    };

    let global = ast::Variable {
        name: global_name,
        info: ast::VariableInfo {
            align: None,
            v_type: ast::Type::Scalar(ast::ScalarType::U32),
            state_space: ast::StateSpace::Global,
            array_init: vec![ast::RegOrImmediate::Imm(ast::ImmediateValue::U64(7))],
        },
    };

    let directives = vec![
        Directive2::Variable(ast::LinkingDirective::VISIBLE, global),
        Directive2::Method(make_kernel(first_kernel_name)),
        Directive2::Method(make_kernel(second_kernel_name)),
    ];

    let plan = build_compilation_plan(directives);

    assert_eq!(plan.kernels.len(), 2);

    for kernel_plan in &plan.kernels {
        assert_eq!(kernel_plan.global_declarations.len(), 1);

        let (linking, declaration) = &kernel_plan.global_declarations[0];

        assert!(linking.contains(ast::LinkingDirective::VISIBLE));
        assert_eq!(declaration.name, global_name);
        assert!(declaration.info.array_init.is_empty());
    }

    assert!(matches!(
        &plan.common[0],
        Directive2::Variable(_, variable)
            if variable.name == global_name
                && variable.info.array_init.len() == 1
    ));
}
