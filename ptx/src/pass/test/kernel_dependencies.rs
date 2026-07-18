use super::super::kernel_dependencies::{
    direct_callees, function_index, kernel_dependencies, kernel_method_sets, reachable_callees,
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
