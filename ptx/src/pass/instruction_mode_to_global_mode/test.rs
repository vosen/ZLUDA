use super::*;
use int_enum::IntEnum;
use strum::EnumCount;

#[repr(usize)]
#[derive(IntEnum, Eq, PartialEq, Copy, Clone, Debug)]
enum Bool {
    False = 0,
    True = 1,
}

fn ftz() -> InstructionModes {
    InstructionModes {
        denormal_f32: Some(DenormalMode::FlushToZero),
        denormal_f16f64: None,
        rounding_f32: None,
        rounding_f16f64: None,
    }
}

fn preserve() -> InstructionModes {
    InstructionModes {
        denormal_f32: Some(DenormalMode::Preserve),
        denormal_f16f64: None,
        rounding_f32: None,
        rounding_f16f64: None,
    }
}

#[test]
fn transitive_mixed() {
    let mut graph = ControlFlowGraph::new();
    let entry_id = SpirvWord(1);
    let false_id = SpirvWord(2);
    let empty_id = SpirvWord(3);
    let false2_id = SpirvWord(4);
    let entry = graph.add_entry_basic_block(entry_id);
    graph.add_jump(entry, false_id);
    let false_ = graph.get_or_add_basic_block(false_id);
    graph.set_modes(false_, ftz(), ftz());
    graph.add_jump(false_, empty_id);
    let empty = graph.get_or_add_basic_block(empty_id);
    graph.add_jump(empty, false2_id);
    let false2_ = graph.get_or_add_basic_block(false2_id);
    graph.set_modes(false2_, ftz(), ftz());
    let partial_result = super::compute_single_mode_insertions(&graph, |node| node.denormal_f32);
    assert_eq!(partial_result.bb_must_insert_mode.len(), 0);
    assert_eq!(partial_result.bb_maybe_insert_mode.len(), 1);
    assert_eq!(
        partial_result.bb_maybe_insert_mode[&false_id],
        (DenormalMode::FlushToZero, iter::once(entry_id).collect())
    );

    let result = optimize_mode_insertions::<DenormalMode, { DenormalMode::COUNT }>(partial_result);
    assert_eq!(result.basic_blocks.len(), 0);
    assert_eq!(result.kernels.len(), 1);
    assert_eq!(result.kernels[&entry_id], DenormalMode::FlushToZero);
}

#[test]
fn transitive_change_twice() {
    let mut graph = ControlFlowGraph::new();
    let entry_id = SpirvWord(1);
    let false_id = SpirvWord(2);
    let empty_id = SpirvWord(3);
    let true_id = SpirvWord(4);
    let entry = graph.add_entry_basic_block(entry_id);
    graph.add_jump(entry, false_id);
    let false_ = graph.get_or_add_basic_block(false_id);
    graph.set_modes(false_, ftz(), ftz());
    graph.add_jump(false_, empty_id);
    let empty = graph.get_or_add_basic_block(empty_id);
    graph.add_jump(empty, true_id);
    let true_ = graph.get_or_add_basic_block(true_id);
    graph.set_modes(true_, preserve(), preserve());
    let partial_result = super::compute_single_mode_insertions(&graph, |node| node.denormal_f32);
    assert_eq!(partial_result.bb_must_insert_mode.len(), 1);
    assert!(partial_result.bb_must_insert_mode.contains(&true_id));
    assert_eq!(partial_result.bb_maybe_insert_mode.len(), 1);
    assert_eq!(
        partial_result.bb_maybe_insert_mode[&false_id],
        (DenormalMode::FlushToZero, iter::once(entry_id).collect())
    );

    let result = optimize_mode_insertions::<DenormalMode, { DenormalMode::COUNT }>(partial_result);
    assert_eq!(result.basic_blocks, iter::once(true_id).collect());
    assert_eq!(result.kernels.len(), 1);
    assert_eq!(result.kernels[&entry_id], DenormalMode::FlushToZero);
}

#[test]
fn transitive_change() {
    let mut graph = ControlFlowGraph::new();
    let entry_id = SpirvWord(1);
    let empty_id = SpirvWord(2);
    let true_id = SpirvWord(3);
    let entry = graph.add_entry_basic_block(entry_id);
    graph.add_jump(entry, empty_id);
    let empty = graph.get_or_add_basic_block(empty_id);
    graph.add_jump(empty, true_id);
    let true_ = graph.get_or_add_basic_block(true_id);
    graph.set_modes(true_, preserve(), preserve());
    let partial_result = super::compute_single_mode_insertions(&graph, |node| node.denormal_f32);
    assert_eq!(partial_result.bb_must_insert_mode.len(), 0);
    assert_eq!(partial_result.bb_maybe_insert_mode.len(), 1);
    assert_eq!(
        partial_result.bb_maybe_insert_mode[&true_id],
        (DenormalMode::Preserve, iter::once(entry_id).collect())
    );

    let result = optimize_mode_insertions::<DenormalMode, { DenormalMode::COUNT }>(partial_result);
    assert_eq!(result.basic_blocks.len(), 0);
    assert_eq!(result.kernels.len(), 1);
    assert_eq!(result.kernels[&entry_id], DenormalMode::Preserve);
}

#[test]
fn codependency() {
    let mut graph = ControlFlowGraph::new();
    let entry_id = SpirvWord(1);
    let left_f_id = SpirvWord(2);
    let right_f_id = SpirvWord(3);
    let left_none_id = SpirvWord(4);
    let mid_none_id = SpirvWord(5);
    let right_none_id = SpirvWord(6);
    let entry = graph.add_entry_basic_block(entry_id);
    graph.add_jump(entry, left_f_id);
    graph.add_jump(entry, right_f_id);
    let left_f = graph.get_or_add_basic_block(left_f_id);
    graph.set_modes(left_f, ftz(), ftz());
    let right_f = graph.get_or_add_basic_block(right_f_id);
    graph.set_modes(right_f, ftz(), ftz());
    graph.add_jump(left_f, left_none_id);
    let left_none = graph.get_or_add_basic_block(left_none_id);
    graph.add_jump(right_f, right_none_id);
    let right_none = graph.get_or_add_basic_block(right_none_id);
    graph.add_jump(left_none, mid_none_id);
    graph.add_jump(right_none, mid_none_id);
    let mid_none = graph.get_or_add_basic_block(mid_none_id);
    graph.add_jump(mid_none, left_none_id);
    graph.add_jump(mid_none, right_none_id);
    //println!(
    //    "{:?}",
    //    petgraph::dot::Dot::with_config(&graph.graph, &[petgraph::dot::Config::EdgeNoLabel])
    //);
    let partial_result = super::compute_single_mode_insertions(&graph, |node| node.denormal_f32);
    assert_eq!(partial_result.bb_must_insert_mode.len(), 0);
    assert_eq!(partial_result.bb_maybe_insert_mode.len(), 2);
    assert_eq!(
        partial_result.bb_maybe_insert_mode[&left_f_id],
        (DenormalMode::FlushToZero, iter::once(entry_id).collect())
    );
    assert_eq!(
        partial_result.bb_maybe_insert_mode[&right_f_id],
        (DenormalMode::FlushToZero, iter::once(entry_id).collect())
    );

    let result = optimize_mode_insertions::<DenormalMode, { DenormalMode::COUNT }>(partial_result);
    assert_eq!(result.basic_blocks.len(), 0);
    assert_eq!(result.kernels.len(), 1);
    assert_eq!(result.kernels[&entry_id], DenormalMode::FlushToZero);
}

static FOLD_DENORMAL_PTX: &'static str = include_str!("fold_denormal.ptx");

#[test]
fn fold_denormal() {
    let method = compile_methods(FOLD_DENORMAL_PTX).pop().unwrap();
    assert_eq!(true, method.flush_to_zero_f32);
    assert_eq!(true, method.flush_to_zero_f16f64);
    let method_body = method.body.unwrap();
    assert!(matches!(
        &*method_body,
        [
            Statement::Label(..),
            Statement::Variable(..),
            Statement::Variable(..),
            Statement::Variable(..),
            Statement::Instruction(ast::Instruction::Add { .. }),
            Statement::Instruction(ast::Instruction::Add { .. }),
            Statement::SetMode(ModeRegister::Denormal {
                f32: false,
                f16f64: false
            }),
            Statement::Instruction(ast::Instruction::Add { .. }),
            Statement::Instruction(ast::Instruction::Add { .. }),
            Statement::Instruction(ast::Instruction::Ret { .. }),
        ]
    ));
}

fn compile_methods(ptx: &str) -> Vec<Function2<ast::Instruction<SpirvWord>, SpirvWord>> {
    use crate::pass::*;

    let module = ptx_parser::parse_module_checked(ptx).unwrap();
    let mut flat_resolver = GlobalStringIdentResolver2::new(SpirvWord(1));
    let mut scoped_resolver = ScopedResolver::new(&mut flat_resolver);
    let directives = normalize_identifiers2::run(&mut scoped_resolver, module.directives).unwrap();
    let directives = normalize_predicates2::run(&mut flat_resolver, directives).unwrap();
    let directives = expand_operands::run(&mut flat_resolver, directives).unwrap();
    let directives = normalize_basic_blocks::run(&mut flat_resolver, directives).unwrap();
    let directives = super::run(&mut flat_resolver, directives).unwrap();
    directives
        .into_iter()
        .filter_map(|s| match s {
            Directive2::Method(m) => Some(m),
            _ => None,
        })
        .collect::<Vec<_>>()
}

static CALL_WITH_MODE_PTX: &'static str = include_str!("call_with_mode.ptx");

#[test]
fn call_with_mode() {
    let methods = compile_methods(CALL_WITH_MODE_PTX);

    assert!(matches!(methods[0].body, None));

    let method_1 = methods[1].body.as_ref().unwrap();
    assert!(matches!(
        &**method_1,
        [
            Statement::Label(..),
            Statement::Variable(..),
            Statement::Instruction(ast::Instruction::Add { .. }),
            Statement::Instruction(ast::Instruction::Call { .. }),
            Statement::Instruction(ast::Instruction::Bra { .. }),
            Statement::Label(..),
            // Dual prelude
            Statement::SetMode(ModeRegister::Denormal {
                f32: true,
                f16f64: true
            }),
            Statement::SetMode(ModeRegister::Rounding {
                f32: ast::RoundingMode::PositiveInf,
                f16f64: ast::RoundingMode::NearestEven
            }),
            Statement::Instruction(ast::Instruction::Bra { .. }),
            // Denormal prelude
            Statement::Label(..),
            Statement::SetMode(ModeRegister::Denormal {
                f32: true,
                f16f64: true
            }),
            Statement::Instruction(ast::Instruction::Bra { .. }),
            // Rounding prelude
            Statement::Label(..),
            Statement::SetMode(ModeRegister::Rounding {
                f32: ast::RoundingMode::PositiveInf,
                f16f64: ast::RoundingMode::NearestEven
            }),
            Statement::Instruction(ast::Instruction::Bra { .. }),
            Statement::Label(..),
            Statement::Instruction(ast::Instruction::Add { .. }),
            Statement::Instruction(ast::Instruction::Ret { .. }),
        ]
    ));
    let [to_fn0] = calls(method_1);
    let [_, dual_prelude, _, _, add] = labels(method_1);
    let [post_call, post_prelude_dual, post_prelude_denormal, post_prelude_rounding] =
        branches(method_1);
    assert_eq!(methods[0].name, to_fn0);
    assert_eq!(post_call, dual_prelude);
    assert_eq!(post_prelude_dual, add);
    assert_eq!(post_prelude_denormal, add);
    assert_eq!(post_prelude_rounding, add);

    let method_2 = methods[2].body.as_ref().unwrap();
    assert!(matches!(
        &**method_2,
        [
            Statement::Label(..),
            Statement::Variable(..),
            Statement::Variable(..),
            Statement::Conditional(..),
            Statement::Label(..),
            Statement::Conditional(..),
            Statement::Label(..),
            Statement::Instruction(ast::Instruction::Bra { .. }),
            Statement::Label(..),
            // Dual prelude
            Statement::SetMode(ModeRegister::Denormal {
                f32: false,
                f16f64: true
            }),
            Statement::SetMode(ModeRegister::Rounding {
                f32: ast::RoundingMode::NegativeInf,
                f16f64: ast::RoundingMode::NearestEven
            }),
            Statement::Instruction(ast::Instruction::Bra { .. }),
            // Denormal prelude
            Statement::Label(..),
            Statement::SetMode(ModeRegister::Denormal {
                f32: false,
                f16f64: true
            }),
            Statement::Instruction(ast::Instruction::Bra { .. }),
            // Rounding prelude
            Statement::Label(..),
            Statement::SetMode(ModeRegister::Rounding {
                f32: ast::RoundingMode::NegativeInf,
                f16f64: ast::RoundingMode::NearestEven
            }),
            Statement::Instruction(ast::Instruction::Bra { .. }),
            Statement::Label(..),
            Statement::Instruction(ast::Instruction::Add { .. }),
            Statement::Instruction(ast::Instruction::Bra { .. }),
            Statement::Label(..),
            Statement::SetMode(ModeRegister::Denormal {
                f32: false,
                f16f64: true
            }),
            Statement::Instruction(ast::Instruction::Bra { .. }),
            Statement::Label(..),
            Statement::Instruction(ast::Instruction::Add { .. }),
            Statement::Instruction(ast::Instruction::Bra { .. }),
            Statement::Label(..),
            Statement::Instruction(ast::Instruction::Ret { .. }),
        ]
    ));
    let [(if_rm_true, if_rm_false), (if_rz_true, if_rz_false)] = conditionals(method_2);
    let [_, conditional2, post_conditional2, prelude_dual, _, _, add1, add2_set_denormal, add2, ret] =
        labels(method_2);
    let [post_conditional2_jump, post_prelude_dual, post_prelude_denormal, post_prelude_rounding, post_add1, post_add2_set_denormal, post_add2] =
        branches(method_2);
    assert_eq!(if_rm_true, prelude_dual);
    assert_eq!(if_rm_false, conditional2);
    assert_eq!(if_rz_true, post_conditional2);
    assert_eq!(if_rz_false, add2_set_denormal);
    assert_eq!(post_conditional2_jump, prelude_dual);
    assert_eq!(post_prelude_dual, add1);
    assert_eq!(post_prelude_denormal, add1);
    assert_eq!(post_prelude_rounding, add1);
    assert_eq!(post_add1, ret);
    assert_eq!(post_add2_set_denormal, add2);
    assert_eq!(post_add2, ret);
}

fn branches<const N: usize>(
    fn_: &Vec<Statement<ast::Instruction<SpirvWord>, SpirvWord>>,
) -> [SpirvWord; N] {
    fn_.iter()
        .filter_map(|s| match s {
            Statement::Instruction(ast::Instruction::Bra {
                arguments: ast::BraArgs { src },
            }) => Some(*src),
            _ => None,
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn labels<const N: usize>(
    fn_: &Vec<Statement<ast::Instruction<SpirvWord>, SpirvWord>>,
) -> [SpirvWord; N] {
    fn_.iter()
        .filter_map(
            |s: &Statement<ptx_parser::Instruction<SpirvWord>, SpirvWord>| match s {
                Statement::Label(label) => Some(*label),
                _ => None,
            },
        )
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn calls<const N: usize>(
    fn_: &Vec<Statement<ast::Instruction<SpirvWord>, SpirvWord>>,
) -> [SpirvWord; N] {
    fn_.iter()
        .filter_map(|s| match s {
            Statement::Instruction(ast::Instruction::Call {
                arguments: ast::CallArgs { func, .. },
                ..
            }) => Some(*func),
            _ => None,
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn conditionals<const N: usize>(
    fn_: &Vec<Statement<ast::Instruction<SpirvWord>, SpirvWord>>,
) -> [(SpirvWord, SpirvWord); N] {
    fn_.iter()
        .filter_map(|s| match s {
            Statement::Conditional(BrachCondition {
                if_true, if_false, ..
            }) => Some((*if_true, *if_false)),
            _ => None,
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}
