// Bulk of the work for raytracing is done across those three transformations:
// * Bounding box needs to convert result pointer from 6xfloat format to
//   8xfloat format. This is done with a very narrow and brittle transformation
//   that looks for very specific assembly instructions
// * Conversions that change state space. This is mostly global -> local
//   conversion and we don't have a way to generically convert argument
//   form one state space to another. So we check only for obvious ones (ld/st).
//   This category splits into two subcategories:
//   * Variables with internally provided semantics (rtLaunchIndex and such),
//     those variables are function arguments, so we convert the instruction
//     state space
//   * Variables with user-provided attributes, they need additional function
//     call to adjust the pointer to use the specific variable
// * Conversion that preserves state space. This is robust and is done only for
//   a pointer to a variable block (so with insertion of variable pointer
//   calculation)

use cpp_demangle::{BorrowedSymbol, DemangleOptions};
use hip_common::raytracing::{Variable, VariablesBlock};
use rustc_hash::{FxHashMap, FxHashSet};
use std::{
    alloc::Layout,
    borrow::Cow,
    cmp,
    collections::BTreeMap,
    convert,
    ffi::{c_void, CStr, CString},
    mem,
};

use crate::{
    ast::{self, VariableDeclaration},
    translate::{
        self, ArgumentDescriptor, BrachCondition, CallGraph, ConstType, ConstantDefinition,
        ExpandedArgParams, ExpandedStatement, Id, IdGenerator, IdNameMapBuilder, LoadVarDetails,
        NormalizedArgParams, PtrAccess, RaytracingEntryPointKind, RaytracingTranslationState,
        RepackVectorDetails, ResolvedCall, Statement, StoreVarDetails, TranslationDirective,
        TranslationMethod, TranslationModule, TypedArgParams, TypedOperand, Visitable,
    },
    TranslateError,
};

static ZLUDA_RT_PTX_IMPL_AMD: &'static [u8] = include_bytes!("../lib/zluda_rt_ptx_impl.bc");
static KERNEL_SOURCE: &'static str = include_str!("../lib/raytracing.cpp");
static KERNEL_HEADER: &'static str = include_str!("../lib/raytracing.hpp");
static KERNEL_SOURCE_BOUNDING_BOX: &'static str =
    include_str!("../lib/raytracing_bounding_box.cpp");
static KERNEL_SOURCE_INTERSECT: &'static str = include_str!("../lib/raytracing_intersect.cpp");
static KERNEL_SOURCE_CALLABLE: &'static str = include_str!("../lib/raytracing_callable.cpp");
const BUILTIN_PREFIX: &'static str = "__zluda_rt_ptx_impl__";

// WARNING: KEEP THIS LIST SYNCHRONIZED WITH zluda_rt_ptx_impl.cpp and raytracing_kernel.cpp
static BUILTINS: &'static [(
    ast::LinkingDirective,
    &'static [ast::Type],
    &'static str,
    &'static [ast::Type],
)] = &[
    (
        ast::LinkingDirective::Extern,
        &[ast::Type::Scalar(ast::ScalarType::B64)],
        "_rt_buffer_get_64",
        &[
            ast::Type::Scalar(ast::ScalarType::B64),
            ast::Type::Scalar(ast::ScalarType::B32),
            ast::Type::Scalar(ast::ScalarType::B32),
            ast::Type::Scalar(ast::ScalarType::B64),
            ast::Type::Scalar(ast::ScalarType::B64),
            ast::Type::Scalar(ast::ScalarType::B64),
            ast::Type::Scalar(ast::ScalarType::B64),
        ],
    ),
    (
        ast::LinkingDirective::Extern,
        &[ast::Type::Scalar(ast::ScalarType::B64)],
        "_rt_buffer_get_id_64",
        &[
            ast::Type::Scalar(ast::ScalarType::B32),
            ast::Type::Scalar(ast::ScalarType::B32),
            ast::Type::Scalar(ast::ScalarType::B32),
            ast::Type::Scalar(ast::ScalarType::B64),
            ast::Type::Scalar(ast::ScalarType::B64),
            ast::Type::Scalar(ast::ScalarType::B64),
            ast::Type::Scalar(ast::ScalarType::B64),
        ],
    ),
    (
        ast::LinkingDirective::Extern,
        &[
            ast::Type::Scalar(ast::ScalarType::B64),
            ast::Type::Scalar(ast::ScalarType::B64),
            ast::Type::Scalar(ast::ScalarType::B64),
            ast::Type::Scalar(ast::ScalarType::B64),
        ],
        "_rt_buffer_get_size_64",
        &[
            ast::Type::Scalar(ast::ScalarType::B64),
            ast::Type::Scalar(ast::ScalarType::B32),
            ast::Type::Scalar(ast::ScalarType::B32),
        ],
    ),
    (
        ast::LinkingDirective::Extern,
        &[
            ast::Type::Scalar(ast::ScalarType::B64),
            ast::Type::Scalar(ast::ScalarType::B64),
            ast::Type::Scalar(ast::ScalarType::B64),
            ast::Type::Scalar(ast::ScalarType::B64),
        ],
        "_rt_buffer_get_id_size_64",
        &[
            ast::Type::Scalar(ast::ScalarType::B32),
            ast::Type::Scalar(ast::ScalarType::B32),
            ast::Type::Scalar(ast::ScalarType::B32),
        ],
    ),
    (
        ast::LinkingDirective::Extern,
        &[],
        "_rt_trace_mask_flags_64",
        &[
            ast::Type::Scalar(ast::ScalarType::B32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::B32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::B32),
            ast::Type::Scalar(ast::ScalarType::B32),
            ast::Type::Scalar(ast::ScalarType::B64),
            ast::Type::Scalar(ast::ScalarType::B32),
        ],
    ),
    (
        ast::LinkingDirective::Extern,
        &[],
        "_rt_trace_time_mask_flags_64",
        &[
            ast::Type::Scalar(ast::ScalarType::B32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::B32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::B32),
            ast::Type::Scalar(ast::ScalarType::B32),
            ast::Type::Scalar(ast::ScalarType::B64),
            ast::Type::Scalar(ast::ScalarType::B32),
        ],
    ),
    (
        ast::LinkingDirective::Extern,
        &[ast::Type::Scalar(ast::ScalarType::B32)],
        "_rt_get_exception_code",
        &[],
    ),
    (
        ast::LinkingDirective::Extern,
        &[ast::Type::Scalar(ast::ScalarType::B32)],
        "_rt_print_active",
        &[],
    ),
    (
        ast::LinkingDirective::Extern,
        &[ast::Type::Scalar(ast::ScalarType::B32)],
        "_rt_potential_intersection",
        &[ast::Type::Scalar(ast::ScalarType::F32)],
    ),
    (
        ast::LinkingDirective::Extern,
        &[ast::Type::Scalar(ast::ScalarType::B32)],
        "_rt_report_intersection",
        &[ast::Type::Scalar(ast::ScalarType::B32)],
    ),
    (ast::LinkingDirective::Extern, &[], "_rt_terminate_ray", &[]),
    (
        ast::LinkingDirective::Extern,
        &[],
        "_rt_ignore_intersection",
        &[],
    ),
    (
        ast::LinkingDirective::Extern,
        &[
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
        ],
        "_rt_transform_tuple",
        &[
            ast::Type::Scalar(ast::ScalarType::B32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
        ],
    ),
    (
        ast::LinkingDirective::Extern,
        &[ast::Type::Scalar(ast::ScalarType::B64)],
        "_rt_callable_program_from_id_64",
        &[ast::Type::Scalar(ast::ScalarType::B32)],
    ),
    (
        ast::LinkingDirective::Extern,
        &[ast::Type::Scalar(ast::ScalarType::B64)],
        "_rt_callable_program_from_id_v2_64",
        &[
            ast::Type::Scalar(ast::ScalarType::B32),
            ast::Type::Scalar(ast::ScalarType::B64),
        ],
    ),
    (
        ast::LinkingDirective::Extern,
        &[
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
        ],
        "_rt_texture_get_f_id",
        &[
            ast::Type::Scalar(ast::ScalarType::B32),
            ast::Type::Scalar(ast::ScalarType::B32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
        ],
    ),
    (
        ast::LinkingDirective::Extern,
        &[
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
        ],
        "_rt_texture_grad_load_or_request_f_id",
        &[
            ast::Type::Scalar(ast::ScalarType::B32),
            ast::Type::Scalar(ast::ScalarType::B32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::B64),
        ],
    ),
    (
        ast::LinkingDirective::Extern,
        &[
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
        ],
        "_rt_texture_lod_load_or_request_f_id",
        &[
            ast::Type::Scalar(ast::ScalarType::B32),
            ast::Type::Scalar(ast::ScalarType::B32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::B64),
        ],
    ),
    (
        ast::LinkingDirective::Extern,
        &[
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
        ],
        "_rt_get_triangle_barycentrics",
        &[],
    ),
    (
        ast::LinkingDirective::Extern,
        &[ast::Type::Scalar(ast::ScalarType::B32)],
        "_rt_get_primitive_index",
        &[],
    ),
    (
        ast::LinkingDirective::Extern,
        &[ast::Type::Scalar(ast::ScalarType::F32)],
        "_rt_is_triangle_hit",
        &[],
    ),
    (
        ast::LinkingDirective::Extern,
        &[ast::Type::Scalar(ast::ScalarType::F32)],
        "_rt_is_triangle_hit_front_face",
        &[],
    ),
    (
        ast::LinkingDirective::Extern,
        &[ast::Type::Scalar(ast::ScalarType::F32)],
        "_rt_is_triangle_hit_back_face",
        &[],
    ),
    (
        ast::LinkingDirective::Extern,
        &[
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::Type::Scalar(ast::ScalarType::F32),
        ],
        "_rt_get_transform",
        &[ast::Type::Scalar(ast::ScalarType::B32)],
    ),
    (
        ast::LinkingDirective::Extern,
        &[],
        "_rt_throw",
        &[ast::Type::Scalar(ast::ScalarType::B32)],
    ),
];

// WARNING: keep in sync with OptiX runtime and LLVM builtins
static BUFFER_OBJECT_FIELDS: &'static [ast::ScalarType] = &[
    ast::ScalarType::B64,
    ast::ScalarType::B64,
    ast::ScalarType::B64,
];

// OptiX emits function calls, but does not emit function declarations
// This poses the problem at several points in the translation process, the
// earliest being the very first pass. Existing code assumes that if a fn name
// is not present it's ok to bail out with an unknown symbol error.
// We solve it below by injecting those declarations before first pass can run.
// An alternative design would be to change first pass to emit a new name
// for unknown symbol. The problem with that solution is we either:
// * Just ignore symbol coherence and create new symbol at first use,
//   this way later passes get a slightly inconsistent. Maybe it's fine, but
//   I like this guarantee
// * Keep list of valid places where a raytracing fn can be used. This has the
//   problem that we need to keep a list of such places. No thanks
// Another alternative would be to inject into AST, but AST uses references to
// input string. We could change this to Cow, but I want to avoid burdening AST
// with data used only during compilation
pub(crate) fn create_module_with_builtins<'input>() -> TranslationModule<'input, NormalizedArgParams>
{
    let mut id_defs = IdNameMapBuilder::new(IdGenerator::new());
    let ptx_impl_imports = BTreeMap::new();
    let directives = BUILTINS
        .into_iter()
        .map(|(_, return_arguments, name, input_arguments)| {
            create_func_declaration(&mut id_defs, true, return_arguments, name, input_arguments).0
        })
        .collect::<Vec<_>>();
    TranslationModule {
        compilation_mode: hip_common::CompilationMode::Wave32,
        sm_version: 0,
        id_defs,
        ptx_impl_imports,
        directives,
    }
}

fn create_func_declaration<'input, P: ast::ArgParams<Id = Id>>(
    id_defs: &mut IdNameMapBuilder<'input>,
    is_replacement: bool,
    return_arguments: &[ast::Type],
    name: &'static str,
    input_arguments: &[ast::Type],
) -> (TranslationDirective<'input, P>, Id) {
    let id_name = id_defs.get_or_add_non_variable(name);
    let return_arguments = return_arguments
        .iter()
        .map(|arg| ast::VariableDeclaration {
            align: None,
            type_: arg.clone(),
            state_space: ast::StateSpace::Reg,
            name: id_defs.id_gen.next(),
        })
        .collect();
    let input_arguments = input_arguments
        .iter()
        .map(|arg| ast::VariableDeclaration {
            align: None,
            type_: arg.clone(),
            state_space: ast::StateSpace::Reg,
            name: id_defs.id_gen.next(),
        })
        .collect();
    (
        TranslationDirective::Method(TranslationMethod {
            return_arguments,
            name: id_name,
            input_arguments,
            body: None,
            tuning: Vec::new(),
            is_kernel: false,
            source_name: if is_replacement {
                Some(Cow::Owned([BUILTIN_PREFIX, name].concat()))
            } else {
                None
            },
            special_raytracing_linking: false,
        }),
        id_name,
    )
}

fn create_func_definition<'input, P: ast::ArgParams<Id = Id>>(
    id_defs: &mut IdNameMapBuilder<'input>,
    return_arguments: &[ast::Type],
    name: &'static str,
    input_arguments: impl Iterator<Item = (ast::Type, ast::StateSpace)>,
) -> (TranslationMethod<'input, P>, String) {
    let source_name = [BUILTIN_PREFIX, name].concat();
    let id_name = id_defs.get_or_add_non_variable(name);
    let return_arguments = return_arguments
        .iter()
        .map(|arg| ast::VariableDeclaration {
            align: None,
            type_: arg.clone(),
            state_space: ast::StateSpace::Reg,
            name: id_defs.register_intermediate(Some((arg.clone(), ast::StateSpace::Reg))),
        })
        .collect();
    let input_arguments = input_arguments
        .map(|(arg, state_space)| ast::VariableDeclaration {
            align: None,
            type_: arg.clone(),
            state_space,
            name: id_defs.register_intermediate(Some((arg, state_space))),
        })
        .collect();
    (
        TranslationMethod {
            return_arguments,
            name: id_name,
            input_arguments,
            body: None,
            tuning: Vec::new(),
            is_kernel: false,
            source_name: Some(Cow::Owned(source_name.clone())),
            special_raytracing_linking: true,
        },
        source_name,
    )
}

pub(crate) fn run_on_normalized<'input>(
    translation_module: TranslationModule<'input, NormalizedArgParams>,
    raytracing_state: &mut RaytracingTranslationState,
) -> Result<TranslationModule<'input, NormalizedArgParams>, TranslateError> {
    let translation_module = transform_entry_point(translation_module, raytracing_state)?;
    convert_arguments_in_bounding_box_kernel(translation_module, raytracing_state)
}

pub(crate) fn run_on_typed<'input, 'a>(
    translation_module: TranslationModule<'input, TypedArgParams>,
    raytracing_state: &'a mut RaytracingTranslationState<'_, 'input>,
) -> Result<TranslationModule<'input, TypedArgParams>, TranslateError> {
    let translation_module = remove_dead_code(translation_module, raytracing_state)?;
    expand_buffer_globals(translation_module, raytracing_state)
}

fn remove_dead_code<'input, 'a>(
    mut translation_module: TranslationModule<'input, TypedArgParams>,
    translation_state: &'a mut RaytracingTranslationState,
) -> Result<TranslationModule<'input, TypedArgParams>, TranslateError> {
    let entry_point = translation_state
        .entry_point_id
        .ok_or_else(TranslateError::unreachable)?;
    let mut call_graph = CallGraph::new(&translation_module.directives[..]);
    let mut reachable_user_functions = call_graph
        .all_callees
        .remove(&entry_point)
        .ok_or_else(TranslateError::unreachable)?;
    reachable_user_functions.insert(entry_point);
    if reachable_user_functions.len() > 1 {
        let entry_point_kind = translation_state
            .entry_point_kind
            .ok_or_else(TranslateError::unreachable)?;
        if entry_point_kind == RaytracingEntryPointKind::BoundingBox {
            return Err(TranslateError::unexpected_pattern());
        }
    }
    translation_module.directives = translation_module
        .directives
        .into_iter()
        .filter(|directive| {
            if let TranslationDirective::Method(method) = directive {
                if method.is_kernel {
                    false
                } else {
                    method.body.is_none() || reachable_user_functions.contains(&method.name)
                }
            } else {
                true
            }
        })
        .collect::<Vec<_>>();
    translation_state.reachable_user_functions = reachable_user_functions;
    Ok(translation_module)
}

fn fixup_indirect_calls<'input, 'a>(
    mut translation_module: TranslationModule<'input, ExpandedArgParams>,
    raytracing_state: &RaytracingTranslationState,
    finalized_args: &FinalizedVariablesLayout,
) -> Result<TranslationModule<'input, ExpandedArgParams>, TranslateError> {
    let id_defs = &mut translation_module.id_defs;
    let directives = &mut translation_module.directives;
    for directive in directives.iter_mut() {
        match directive {
            TranslationDirective::Variable(..) => {}
            TranslationDirective::Method(method) => {
                let injected_args = finalized_args.injected_args.get(&method.name);
                if let Some(ref mut method_body) = method.body {
                    let injected_args = injected_args.ok_or_else(TranslateError::unreachable)?;
                    let old_statements = mem::take(method_body);
                    let mut new_statements = Vec::with_capacity(old_statements.len());
                    for statement in old_statements {
                        match statement {
                            Statement::Call(mut call) if call.is_indirect => {
                                let global_state = InjectedArgument::GlobalState.get_type_space();
                                let launch_index = InjectedArgument::LaunchIndex.get_type_space();
                                let exception_code = id_defs.register_intermediate(Some((
                                    ast::Type::Scalar(ast::ScalarType::B32),
                                    ast::StateSpace::Reg,
                                )));
                                call.return_arguments.push((
                                    exception_code,
                                    ast::Type::Scalar(ast::ScalarType::B32),
                                    ast::StateSpace::Reg,
                                ));
                                call.input_arguments.push((
                                    injected_args
                                        .get(InjectedArgument::GlobalState)
                                        .ok_or_else(TranslateError::unreachable)?,
                                    global_state.0,
                                    global_state.1,
                                ));
                                call.input_arguments.push((
                                    injected_args
                                        .get(InjectedArgument::LaunchIndex)
                                        .ok_or_else(TranslateError::unreachable)?,
                                    launch_index.0,
                                    launch_index.1,
                                ));
                                let src = call.name;
                                let ld_dst = id_defs.register_intermediate(Some((
                                    ast::Type::Scalar(ast::ScalarType::B64),
                                    ast::StateSpace::Reg,
                                )));
                                let ld = Statement::Instruction(ast::Instruction::Ld(
                                    ast::LdDetails {
                                        qualifier: ast::LdStQualifier::Weak,
                                        state_space: ast::StateSpace::Global,
                                        caching: ast::LdCacheOperator::Cached,
                                        typ: ast::Type::Scalar(ast::ScalarType::B64),
                                        non_coherent: false,
                                    },
                                    ast::Arg2Ld { dst: ld_dst, src },
                                ));
                                call.name = ld_dst;
                                call.input_arguments.push((
                                    src,
                                    ast::Type::Scalar(ast::ScalarType::B8),
                                    ast::StateSpace::Global,
                                ));
                                new_statements.push(ld);
                                new_statements.push(Statement::Call(call));
                                interpret_exception_code(
                                    id_defs,
                                    raytracing_state,
                                    &*method.return_arguments,
                                    &mut new_statements,
                                    exception_code,
                                )?;
                            }
                            statement => new_statements.push(statement),
                        }
                    }
                    *method_body = new_statements;
                };
            }
        }
    }
    Ok(translation_module)
}

fn interpret_exception_code(
    id_defs: &mut IdNameMapBuilder,
    raytracing_state: &RaytracingTranslationState,
    return_arguments: &[VariableDeclaration<Id>],
    new_statements: &mut Vec<ExpandedStatement>,
    exception_code: Id,
) -> Result<(), TranslateError> {
    match raytracing_state.entry_point_kind {
        Some(RaytracingEntryPointKind::Callable | RaytracingEntryPointKind::Unknown) => {}
        _ => return Err(TranslateError::todo()),
    };
    insert_call_with_exception_propagation(
        id_defs,
        new_statements,
        exception_code,
        |id_defs, body| {
            zeroed_return_vector(
                id_defs,
                &return_arguments[..return_arguments.len() - 1],
                body,
            )
        },
    )
}

// Buffer declarations in PTX look like this:
//      .global .align 1 .b8 result_buffer[1];
// and then are used like this:
//      mov.u64 	%rd7, result_buffer;
//      cvta.global.u64 	%rd2, %rd7;
//      call (%rd1), _rt_buffer_get_64, (%rd2, %r1, %r2, %rd3, %rd4, %rd6, %rd6);
// We simply detect this pattern and then convert the type of the global
// TODO: Currently, we detect only this exact pattern and do it badly, but it
// should be made more robust by checking if dst registers (in the example
// above %rd2 and %rd7) are SSA
fn expand_buffer_globals<'input, 'a>(
    mut translation_module: TranslationModule<'input, TypedArgParams>,
    variables: &'a mut RaytracingTranslationState<'_, 'input>,
) -> Result<TranslationModule<'input, TypedArgParams>, TranslateError> {
    static BUFFER_BUILTINS: &[&'static str] = &["_rt_buffer_get_64", "_rt_buffer_get_size_64"];
    let buffer_functions =
        get_rt_functions_with_undecorated_names(&translation_module, BUFFER_BUILTINS)?;
    let mut potential_buffers = FxHashMap::default();
    let mut address_reads = FxHashMap::default();
    let mut address_conversions = FxHashMap::default();
    let mut buffer_call_arguments = FxHashSet::default();
    for (index, directive) in translation_module.directives.iter().enumerate() {
        match directive {
            TranslationDirective::Variable(
                _,
                _,
                translate::Variable {
                    align: Some(1),
                    type_: ast::Type::Array(ast::ScalarType::B8, array_dims),
                    state_space: ast::StateSpace::Global,
                    name,
                    initializer,
                },
            ) => {
                if &*array_dims == &[1] && initializer.is_none() {
                    potential_buffers.insert(*name, (index, false));
                }
            }
            TranslationDirective::Method(method) => {
                for statement in method.body.iter().flatten() {
                    match statement {
                        Statement::Call(ResolvedCall {
                            name,
                            input_arguments,
                            ..
                        }) => {
                            if buffer_functions.contains(name) {
                                if let TypedOperand::Reg(reg) = input_arguments[0].0 {
                                    buffer_call_arguments.insert(reg);
                                }
                            }
                        }
                        Statement::Instruction(ast::Instruction::Cvta(
                            ast::CvtaDetails {
                                from: ast::StateSpace::Global,
                                to: ast::StateSpace::Generic,
                                size: ast::CvtaSize::U64,
                            },
                            ast::Arg2 {
                                dst: TypedOperand::Reg(dst),
                                src: TypedOperand::Reg(src),
                            },
                        )) => {
                            address_conversions.insert(*dst, *src);
                        }
                        Statement::Instruction(ast::Instruction::Mov(
                            ast::MovDetails {
                                typ: ast::Type::Scalar(ast::ScalarType::U64),
                                ..
                            },
                            ast::Arg2Mov {
                                dst: TypedOperand::Reg(dst),
                                src: TypedOperand::Reg(src),
                            },
                        )) => {
                            address_reads.insert(*dst, *src);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    // Working backwards from calls is slightly more robust, because it's more
    // likely that destination register is written into only once
    for arg in buffer_call_arguments {
        if let Some(cvta_src) = address_conversions.get(&arg) {
            if let Some(mov_src) = address_reads.get(cvta_src) {
                if let Some((_, is_buffer)) = potential_buffers.get_mut(mov_src) {
                    *is_buffer = true;
                }
            }
        }
    }
    for (_, (directive_idx, is_buffer)) in potential_buffers {
        if is_buffer {
            match translation_module
                .directives
                .get_mut(directive_idx)
                .unwrap()
            {
                TranslationDirective::Variable(_, _, translate::Variable { type_, name, .. }) => {
                    let str_name = translation_module
                        .id_defs
                        .globals
                        .reverse_variables
                        .get(name)
                        .cloned()
                        .ok_or_else(TranslateError::unreachable)?;
                    variables.buffers.insert(*name, str_name);
                    *type_ = ast::Type::Struct(
                        BUFFER_OBJECT_FIELDS
                            .iter()
                            .copied()
                            .map(ast::StructField::Scalar)
                            .collect(),
                    )
                }
                _ => return Err(TranslateError::unreachable()),
            }
        }
    }
    Ok(translation_module)
}

fn get_rt_functions_with_undecorated_names(
    module: &TranslationModule<TypedArgParams>,
    names: &[&'static str],
) -> Result<Vec<Id>, TranslateError> {
    let mut result = Vec::new();
    for directive in module.directives.iter() {
        if let TranslationDirective::Method(method) = directive {
            if let Some(ref name) = method.source_name {
                if name.starts_with(BUILTIN_PREFIX)
                    && names.contains(&&name[BUILTIN_PREFIX.len()..])
                {
                    result.push(method.name);
                    if names.len() == result.len() {
                        return Ok(result);
                    }
                }
            }
        }
    }
    Err(TranslateError::unreachable())
}

pub(crate) fn postprocess<'a, 'ast>(
    translation_module: TranslationModule<'ast, ExpandedArgParams>,
    variables: &mut RaytracingTranslationState<'a, 'ast>,
) -> Result<TranslationModule<'ast, ExpandedArgParams>, TranslateError> {
    let (translation_module, generated_layout) = convert_global_variables(
        translation_module,
        variables,
        variables
            .entry_point_id
            .ok_or_else(TranslateError::unreachable)?,
    )?;
    let translation_module: TranslationModule<'_, ExpandedArgParams> =
        replace_exception_functions(translation_module, &variables, &generated_layout)?;
    let translation_module =
        fixup_indirect_calls(translation_module, &variables, &generated_layout)?;
    let translation_module =
        generate_attribute_wrapper(translation_module, variables, &generated_layout)?;
    let translation_module = expand_rt_functions(translation_module, &generated_layout)?;
    let translation_module = replace_trace_control_functions(translation_module, &variables)?;
    let translation_module = replace_attribute_functions(translation_module, &generated_layout)?;
    variables.variables = generated_layout.variables.globals;
    variables.new_attribute_variables = generated_layout.variables.user_attributes;
    Ok(translation_module)
}

// TODO: support throwing from intersection and bounding box
fn replace_exception_functions<'input>(
    mut translation_module: TranslationModule<'input, ExpandedArgParams>,
    raytracing_state: &RaytracingTranslationState,
    var_tracker: &FinalizedVariablesLayout<'input>,
) -> Result<TranslationModule<'input, ExpandedArgParams>, TranslateError> {
    let trace =
        inject_return_arg_into_trace_fn(&mut translation_module, "_rt_trace_mask_flags_64")?;
    let trace_time =
        inject_return_arg_into_trace_fn(&mut translation_module, "_rt_trace_time_mask_flags_64")?;
    match raytracing_state.entry_point_kind {
        Some(RaytracingEntryPointKind::Unknown | RaytracingEntryPointKind::Callable) => {}
        _ => return Ok(translation_module),
    };
    let get_exception_code = translation_module
        .id_defs
        .globals
        .variables
        .get("_rt_get_exception_code")
        .copied()
        .ok_or_else(TranslateError::unreachable)?;
    let throw = translation_module
        .id_defs
        .globals
        .variables
        .get("_rt_throw")
        .copied()
        .ok_or_else(TranslateError::unreachable)?;
    let modified_funcs = get_functions_with_bodies(&translation_module.directives)?;
    let mut fixup_declarations = FxHashSet::default();
    for directive in translation_module.directives.iter_mut() {
        if let TranslationDirective::Method(method) = directive {
            let old_body = match &mut method.body {
                Some(body) => {
                    let name = translation_module.id_defs.register_intermediate(Some((
                        ast::Type::Scalar(ast::ScalarType::B32),
                        ast::StateSpace::Reg,
                    )));
                    method.return_arguments.push(ast::VariableDeclaration {
                        align: None,
                        type_: ast::Type::Scalar(ast::ScalarType::B32),
                        state_space: ast::StateSpace::Reg,
                        name,
                    });
                    body.drain(..)
                }
                None => continue,
            };
            fixup_declarations.insert(method.name);
            let mut new_body = Vec::with_capacity(old_body.len());
            for statement in old_body {
                match statement {
                    Statement::Call(call) if call.name == get_exception_code => {
                        let (dst, _, _) = call
                            .return_arguments
                            .get(0)
                            .ok_or_else(TranslateError::unexpected_pattern)?;
                        let injected_args = var_tracker
                            .injected_args
                            .get(&method.name)
                            .ok_or_else(TranslateError::unreachable)?;
                        let prim_idx = injected_args
                            .get(InjectedArgument::PrimitiveIndex)
                            .ok_or_else(TranslateError::unreachable)?;
                        new_body.push(Statement::Instruction(ast::Instruction::Mov(
                            ast::MovDetails {
                                typ: ast::Type::Scalar(ast::ScalarType::B32),
                                dst_width: 0,
                                src_width: 0,
                                relaxed_src2_conv: false,
                            },
                            ast::Arg2Mov {
                                dst: *dst,
                                src: prim_idx,
                            },
                        )))
                    }
                    Statement::Call(call) if call.name == throw => {
                        let mut returned = zeroed_return_vector(
                            &mut translation_module.id_defs,
                            &method.return_arguments[..method.return_arguments.len() - 1],
                            &mut new_body,
                        )?;
                        let (error, _, _) = call
                            .input_arguments
                            .get(0)
                            .ok_or_else(TranslateError::unexpected_pattern)?;
                        returned.push((*error, ast::Type::Scalar(ast::ScalarType::B32)));
                        new_body.push(Statement::RetValue(
                            ast::RetData { uniform: false },
                            returned,
                        ))
                    }
                    Statement::Call(
                        mut call @ ResolvedCall {
                            name,
                            is_indirect: false,
                            ..
                        },
                    ) if name == trace || name == trace_time || modified_funcs.contains(&name) => {
                        let dst = translation_module.id_defs.register_intermediate(Some((
                            ast::Type::Scalar(ast::ScalarType::B32),
                            ast::StateSpace::Reg,
                        )));
                        call.return_arguments.push((
                            dst,
                            ast::Type::Scalar(ast::ScalarType::B32),
                            ast::StateSpace::Reg,
                        ));
                        new_body.push(Statement::Call(call));
                        let return_arguments = &method.return_arguments;
                        insert_call_with_exception_propagation(
                            &mut translation_module.id_defs,
                            &mut new_body,
                            dst,
                            |id_defs, body| {
                                zeroed_return_vector(
                                    id_defs,
                                    &return_arguments[..return_arguments.len() - 1],
                                    body,
                                )
                            },
                        )?;
                    }
                    Statement::RetValue(ret, mut returned) => {
                        let zero_id = translation_module.id_defs.register_intermediate(Some((
                            ast::Type::Scalar(ast::ScalarType::B32),
                            ast::StateSpace::Reg,
                        )));
                        let constant = Statement::Constant(ConstantDefinition {
                            dst: zero_id,
                            typ: ast::ScalarType::B32,
                            value: ast::ImmediateValue::U64(0),
                        });
                        returned.push((zero_id, ast::Type::Scalar(ast::ScalarType::B32)));
                        new_body.extend([constant, Statement::RetValue(ret, returned)]);
                    }
                    Statement::Instruction(ast::Instruction::Ret(ret)) => {
                        let zero_id = translation_module.id_defs.register_intermediate(Some((
                            ast::Type::Scalar(ast::ScalarType::B32),
                            ast::StateSpace::Reg,
                        )));
                        let constant = Statement::Constant(ConstantDefinition {
                            dst: zero_id,
                            typ: ast::ScalarType::B32,
                            value: ast::ImmediateValue::U64(0),
                        });
                        let returned = vec![(zero_id, ast::Type::Scalar(ast::ScalarType::B32))];
                        new_body.extend([constant, Statement::RetValue(ret, returned)]);
                    }
                    s => new_body.push(s),
                }
            }
            method.body = Some(new_body);
        }
    }
    // This is a disgusting hack that should be done better
    for directive in translation_module.directives.iter_mut() {
        if let TranslationDirective::Method(TranslationMethod {
            body: None,
            return_arguments,
            name,
            ..
        }) = directive
        {
            if !fixup_declarations.contains(name) {
                continue;
            }
            let name = translation_module.id_defs.register_intermediate(Some((
                ast::Type::Scalar(ast::ScalarType::B32),
                ast::StateSpace::Reg,
            )));
            return_arguments.push(ast::VariableDeclaration {
                align: None,
                type_: ast::Type::Scalar(ast::ScalarType::B32),
                state_space: ast::StateSpace::Reg,
                name,
            });
        }
    }
    Ok(translation_module)
}

fn get_functions_with_bodies(
    directives: &[TranslationDirective<ExpandedArgParams>],
) -> Result<FxHashSet<Id>, TranslateError> {
    let mut result = FxHashSet::default();
    for directive in directives {
        match directive {
            TranslationDirective::Variable(..) => continue,
            TranslationDirective::Method(method) => {
                if method.body.is_some() {
                    result.insert(method.name);
                }
            }
        }
    }
    Ok(result)
}

fn zeroed_return_vector<'input>(
    id_defs: &mut IdNameMapBuilder<'input>,
    return_args: &[ast::VariableDeclaration<Id>],
    body: &mut Vec<ExpandedStatement>,
) -> Result<Vec<(Id, ast::Type)>, TranslateError> {
    return_args
        .iter()
        .map(|arg| Ok((zeroed(id_defs, body, &arg.type_)?, arg.type_.clone())))
        .collect::<Result<Vec<_>, _>>()
}

fn zeroed<'input>(
    id_defs: &mut IdNameMapBuilder<'input>,
    body: &mut Vec<ExpandedStatement>,
    type_: &ast::Type,
) -> Result<Id, TranslateError> {
    match type_ {
        ast::Type::Scalar(ast::ScalarType::Pred)
        | ast::Type::Vector(..)
        | ast::Type::Pointer(..)
        | ast::Type::Texref
        | ast::Type::Surfref
        | ast::Type::Struct(..) => Err(TranslateError::unexpected_pattern()),
        ast::Type::Scalar(scalar) => {
            let dst = id_defs.register_intermediate(Some((type_.clone(), ast::StateSpace::Reg)));
            let value = if scalar.kind() == ast::ScalarKind::Float {
                ast::ImmediateValue::F64(0f64)
            } else {
                ast::ImmediateValue::U64(0u64)
            };
            body.push(Statement::Constant(ConstantDefinition {
                dst,
                typ: *scalar,
                value,
            }));
            Ok(dst)
        }
        ast::Type::Array(scalar, dims) => {
            let value = if scalar.kind() == ast::ScalarKind::Float {
                ast::ImmediateValue::F64(0f64)
            } else {
                ast::ImmediateValue::U64(0u64)
            };
            let name = id_defs.register_intermediate(Some((
                ast::Type::Array(*scalar, dims.clone()),
                ast::StateSpace::Reg,
            )));
            let (last_dims, rest_dims) =
                dims.split_last().ok_or_else(TranslateError::unreachable)?;
            let innermost_dim = ast::Initializer::Array(vec![
                ast::Initializer::Constant(value);
                *last_dims as usize
            ]);
            let initializer = rest_dims.iter().rev().fold(innermost_dim, |init, dim| {
                ast::Initializer::Array(vec![init; *dim as usize])
            });
            body.push(Statement::Variable(translate::Variable {
                align: None,
                type_: ast::Type::Array(*scalar, dims.clone()),
                state_space: ast::StateSpace::Local,
                name,
                initializer: Some(initializer),
            }));
            let dst = id_defs.register_intermediate(Some((
                ast::Type::Array(*scalar, dims.clone()),
                ast::StateSpace::Reg,
            )));
            body.push(Statement::LoadVar(LoadVarDetails {
                arg: ast::Arg2 { dst, src: name },
                typ: ast::Type::Array(*scalar, dims.clone()),
                _state_space: ast::StateSpace::Local,
                member_index: None,
            }));
            Ok(dst)
        }
    }
}

fn insert_call_with_exception_propagation<'input>(
    id_defs: &mut IdNameMapBuilder<'input>,
    new_body: &mut Vec<ExpandedStatement>,
    exception_code: Id,
    default_ret: impl FnOnce(
        &mut IdNameMapBuilder<'input>,
        &mut Vec<ExpandedStatement>,
    ) -> Result<Vec<(Id, ast::Type)>, TranslateError>,
) -> Result<(), TranslateError> {
    let constant = id_defs.register_intermediate(Some((
        ast::Type::Scalar(ast::ScalarType::B32),
        ast::StateSpace::Reg,
    )));
    let constant_instruction = Statement::Constant(ConstantDefinition {
        dst: constant,
        typ: ast::ScalarType::B32,
        value: ast::ImmediateValue::U64(1024),
    });
    let early_return = id_defs.register_intermediate(Some((
        ast::Type::Scalar(ast::ScalarType::Pred),
        ast::StateSpace::Reg,
    )));
    let return_check = Statement::Instruction(ast::Instruction::Setp(
        ast::SetpData {
            typ: ast::ScalarType::B32,
            flush_to_zero: None,
            cmp_op: ast::SetpCompareOp::GreaterOrEq,
        },
        ast::Arg4Setp {
            dst1: early_return,
            dst2: None,
            src1: exception_code,
            src2: constant,
        },
    ));
    let return_label_id = id_defs.register_intermediate(None);
    let return_label = Statement::Label(return_label_id);
    let continue_label_id = id_defs.register_intermediate(None);
    let continue_label = Statement::Label(continue_label_id);
    let return_jmp = Statement::Conditional(BrachCondition {
        predicate: early_return,
        if_true: return_label_id,
        if_false: continue_label_id,
    });
    let mut returned = default_ret(id_defs, new_body)?;
    returned.push((exception_code, ast::Type::Scalar(ast::ScalarType::B32)));
    let return_ = Statement::RetValue(ast::RetData { uniform: false }, returned);
    new_body.extend([
        constant_instruction,
        return_check,
        return_jmp,
        return_label,
        return_,
        continue_label,
    ]);
    Ok(())
}

fn inject_return_arg_into_trace_fn(
    translation_module: &mut TranslationModule<ExpandedArgParams>,
    name: &str,
) -> Result<Id, TranslateError> {
    let fn_ = translation_module
        .id_defs
        .globals
        .variables
        .get(name)
        .copied()
        .ok_or_else(TranslateError::unreachable)?;

    for directive in translation_module.directives.iter_mut() {
        if let TranslationDirective::Method(method) = directive {
            if method.name != fn_ {
                continue;
            }
            let name = translation_module.id_defs.register_intermediate(Some((
                ast::Type::Scalar(ast::ScalarType::B32),
                ast::StateSpace::Reg,
            )));
            method.return_arguments.push(ast::VariableDeclaration {
                align: None,
                type_: ast::Type::Scalar(ast::ScalarType::B32),
                state_space: ast::StateSpace::Reg,
                name,
            });
            return Ok(fn_);
        }
    }
    Err(TranslateError::unreachable())
}

// TODO: replace in subfunctions
fn replace_trace_control_functions<'input>(
    mut translation_module: TranslationModule<'input, ExpandedArgParams>,
    raytracing_state: &RaytracingTranslationState,
) -> Result<TranslationModule<'input, ExpandedArgParams>, TranslateError> {
    if raytracing_state.entry_point_kind != Some(RaytracingEntryPointKind::Unknown) {
        return Ok(translation_module);
    }
    let ignore_intersection = translation_module
        .id_defs
        .globals
        .variables
        .get("_rt_ignore_intersection")
        .copied()
        .ok_or_else(TranslateError::unreachable)?;
    let terminate_ray = translation_module
        .id_defs
        .globals
        .variables
        .get("_rt_terminate_ray")
        .copied()
        .ok_or_else(TranslateError::unreachable)?;
    let entry_point = raytracing_state
        .entry_point_id
        .ok_or_else(TranslateError::unreachable)?;
    let func = translation_module
        .directives
        .iter_mut()
        .find_map(|directive| match directive {
            TranslationDirective::Variable(..) => None,
            TranslationDirective::Method(method) => {
                if method.name == entry_point {
                    Some(method)
                } else {
                    None
                }
            }
        })
        .ok_or_else(TranslateError::unreachable)?;
    let mut new_body = Vec::new();
    let old_body = func.body.take().ok_or_else(TranslateError::unreachable)?;
    for statement in old_body.into_iter() {
        match statement {
            Statement::Instruction(ast::Instruction::Ret(ret)) => {
                let constant_0 = translation_module.id_defs.register_intermediate(Some((
                    ast::Type::Scalar(ast::ScalarType::B32),
                    ast::StateSpace::Reg,
                )));
                new_body.push(ExpandedStatement::Constant(ConstantDefinition {
                    dst: constant_0,
                    typ: ast::ScalarType::B32,
                    value: ast::ImmediateValue::U64(0),
                }));
                new_body.push(Statement::RetValue(
                    ret,
                    vec![(constant_0, ast::Type::Scalar(ast::ScalarType::B32))],
                ));
            }
            Statement::Call(ResolvedCall { name, .. }) if name == ignore_intersection => {
                let constant_1 = translation_module.id_defs.register_intermediate(Some((
                    ast::Type::Scalar(ast::ScalarType::B32),
                    ast::StateSpace::Reg,
                )));
                new_body.push(ExpandedStatement::Constant(ConstantDefinition {
                    dst: constant_1,
                    typ: ast::ScalarType::B32,
                    value: ast::ImmediateValue::U64(1),
                }));
                new_body.push(Statement::RetValue(
                    ast::RetData { uniform: false },
                    vec![(constant_1, ast::Type::Scalar(ast::ScalarType::B32))],
                ));
            }
            Statement::Call(ResolvedCall { name, .. }) if name == terminate_ray => {
                let constant_2 = translation_module.id_defs.register_intermediate(Some((
                    ast::Type::Scalar(ast::ScalarType::B32),
                    ast::StateSpace::Reg,
                )));
                new_body.push(ExpandedStatement::Constant(ConstantDefinition {
                    dst: constant_2,
                    typ: ast::ScalarType::B32,
                    value: ast::ImmediateValue::U64(2),
                }));
                new_body.push(Statement::RetValue(
                    ast::RetData { uniform: false },
                    vec![(constant_2, ast::Type::Scalar(ast::ScalarType::B32))],
                ));
            }
            statement => new_body.push(statement),
        }
    }
    func.body = Some(new_body);
    Ok(translation_module)
}

fn generate_attribute_wrapper<'input>(
    mut translation_module: TranslationModule<'input, ExpandedArgParams>,
    raytracing_state: &mut RaytracingTranslationState,
    finalized_args: &FinalizedVariablesLayout<'input>,
) -> Result<TranslationModule<'input, ExpandedArgParams>, TranslateError> {
    let get_var_ptr_global = translation_module
        .id_defs
        .globals
        .variables
        .get("__zluda_rt_ptx_impl__get_variable_pointer_global")
        .copied()
        .ok_or_else(TranslateError::unreachable)?;
    let entry_point = raytracing_state
        .entry_point_id
        .ok_or_else(TranslateError::unreachable)?;
    let intersection = match raytracing_state.entry_point_kind {
        None => return Err(TranslateError::unreachable()),
        Some(RaytracingEntryPointKind::Unknown) => false,
        Some(RaytracingEntryPointKind::Intersection) => true,
        _ => return Ok(translation_module),
    };
    let mut original_input_arguments = None;
    for directive in translation_module.directives.iter() {
        match directive {
            TranslationDirective::Variable(..) => continue,
            TranslationDirective::Method(method) => {
                if method.name != entry_point {
                    continue;
                }
                let input_arguments = method
                    .input_arguments
                    .iter()
                    .map(|arg| (arg.name, arg.type_.clone(), arg.state_space))
                    .collect::<Vec<_>>();
                original_input_arguments = Some(input_arguments);
                break;
            }
        }
    }
    let original_input_arguments =
        original_input_arguments.ok_or_else(TranslateError::unreachable)?;
    let entry_injected_args = finalized_args
        .injected_args
        .get(&entry_point)
        .ok_or_else(TranslateError::unreachable)?;
    let attribute_block_index = injected_argument_index(
        entry_injected_args,
        InjectedArgument::AttributesBlock,
        &original_input_arguments,
    )?;
    let variables_block_index = injected_argument_index(
        entry_injected_args,
        InjectedArgument::VariablesBlock,
        &original_input_arguments,
    )?;
    let (mut method, wrapper_name) = create_func_definition(
        &mut translation_module.id_defs,
        if intersection {
            &[]
        } else {
            &[ast::Type::Scalar(ast::ScalarType::B32)]
        },
        "rollback_wrapper",
        original_input_arguments
            .iter()
            .map(|(_, var_type, state_space)| (var_type.clone(), *state_space)),
    );
    let maybe_anyhit_pointer_arg = if intersection {
        None
    } else {
        let anyhit_pointer_arg = translation_module.id_defs.register_intermediate(Some((
            ast::Type::Scalar(ast::ScalarType::B64),
            ast::StateSpace::Reg,
        )));
        method.input_arguments.push(ast::VariableDeclaration {
            align: None,
            type_: ast::Type::Scalar(ast::ScalarType::B64),
            state_space: ast::StateSpace::Reg,
            name: anyhit_pointer_arg,
        });
        Some(anyhit_pointer_arg)
    };
    let mut body = Vec::new();
    let attribute_block = method
        .input_arguments
        .get(attribute_block_index)
        .ok_or_else(TranslateError::unreachable)?
        .name;
    let original_attribute_values = insert_attribute_prologue(
        &mut translation_module.id_defs,
        &mut body,
        &finalized_args.variables,
        get_var_ptr_global,
        attribute_block,
    )?;
    let input_args_to_copy = if intersection {
        method.input_arguments.len()
    } else {
        method.input_arguments.len() - 1
    };
    insert_attribute_call(
        &mut translation_module.id_defs,
        &mut body,
        entry_point,
        intersection,
        &method.input_arguments[..input_args_to_copy],
        None,
    )?;
    let (control_variable, rollback_constant) =
        if let Some(anyhit_pointer_arg) = maybe_anyhit_pointer_arg {
            let anyhit_result = insert_attribute_call(
                &mut translation_module.id_defs,
                &mut body,
                anyhit_pointer_arg,
                false,
                &method.input_arguments[..method.input_arguments.len() - 1],
                Some(variables_block_index),
            )?;
            (anyhit_result, 1)
        } else {
            let intersection_result_index = injected_argument_index(
                entry_injected_args,
                InjectedArgument::IntersectionResult,
                &original_input_arguments,
            )?;
            let intersection_result = method
                .input_arguments
                .get(intersection_result_index)
                .ok_or_else(TranslateError::unreachable)?
                .name;
            let loaded_intersection_result = insert_attribute_intersection_result_load(
                &mut translation_module.id_defs,
                &mut body,
                intersection_result,
            );
            (loaded_intersection_result, 2)
        };
    let return_label = translation_module.id_defs.register_intermediate(None);
    insert_attribute_anyhit_check_rollback_jmp(
        &mut translation_module.id_defs,
        &mut body,
        control_variable,
        return_label,
        rollback_constant,
    );
    insert_attribute_rollback(
        &mut translation_module.id_defs,
        &mut body,
        get_var_ptr_global,
        attribute_block,
        original_attribute_values,
    );
    body.push(Statement::Label(return_label));
    if intersection {
        body.push(Statement::Instruction(ast::Instruction::Ret(
            ast::RetData { uniform: false },
        )));
    } else {
        body.push(Statement::RetValue(
            ast::RetData { uniform: false },
            vec![(control_variable, ast::Type::Scalar(ast::ScalarType::B32))],
        ));
    }
    method.body = Some(body);
    translation_module
        .directives
        .push(TranslationDirective::Method(method));
    if intersection {
        raytracing_state.kernel_name = wrapper_name;
    }
    Ok(translation_module)
}

fn insert_attribute_intersection_result_load(
    id_defs: &mut IdNameMapBuilder,
    body: &mut Vec<Statement<ast::Instruction<ExpandedArgParams>, ExpandedArgParams>>,
    intersection_result: Id,
) -> Id {
    let (type_, state_space) = InjectedArgument::IntersectionResult.get_type_space();
    let dst = id_defs.register_intermediate(Some((type_.clone(), ast::StateSpace::Reg)));
    body.push(Statement::Instruction(ast::Instruction::Ld(
        ast::LdDetails {
            qualifier: ast::LdStQualifier::Weak,
            caching: ast::LdCacheOperator::Cached,
            non_coherent: false,
            state_space,
            typ: type_,
        },
        ast::Arg2Ld {
            dst,
            src: intersection_result,
        },
    )));
    dst
}

fn injected_argument_index(
    entry_injected_args: &InjectedArguments,
    kind: InjectedArgument,
    original_input_arguments: &Vec<(Id, ast::Type, ast::StateSpace)>,
) -> Result<usize, TranslateError> {
    let original_id = entry_injected_args
        .get(kind)
        .ok_or_else(TranslateError::unreachable)?;
    let index = original_input_arguments
        .iter()
        .enumerate()
        .find_map(|(index, (name, _, _))| {
            if *name == original_id {
                Some(index)
            } else {
                None
            }
        })
        .ok_or_else(TranslateError::unreachable)?;
    Ok(index)
}

fn insert_attribute_prologue<'input>(
    id_defs: &mut IdNameMapBuilder<'input>,
    result: &mut Vec<ExpandedStatement>,
    var_tracker: &VariablesLayout<'input>,
    get_var_ptr_global: Id,
    attribute_block: Id,
) -> Result<Vec<(Id, ast::Type, u32)>, TranslateError> {
    let mut new_values = Vec::new();
    for (id, sema) in var_tracker.tracker.global_to_semantic.iter() {
        let (type_, offset) = if let VariableSemantic::Attribute { type_, .. } = sema {
            let offset = var_tracker
                .offsets
                .get(id)
                .ok_or_else(TranslateError::unreachable)?;
            (type_, *offset)
        } else {
            continue;
        };
        let var_ptr = insert_call_to_get_variable_pointer(
            id_defs,
            attribute_block,
            (get_var_ptr_global, ast::StateSpace::Global),
            result,
            offset,
        );
        let dst = id_defs.register_intermediate(Some((type_.clone(), ast::StateSpace::Reg)));
        // TODO: use alignment
        result.push(ExpandedStatement::Instruction(ast::Instruction::Ld(
            ast::LdDetails {
                qualifier: ast::LdStQualifier::Weak,
                state_space: ast::StateSpace::Global,
                caching: ast::LdCacheOperator::Cached,
                typ: type_.clone(),
                non_coherent: false,
            },
            ast::Arg2Ld { dst, src: var_ptr },
        )));
        new_values.push((dst, type_.clone(), offset));
    }
    Ok(new_values)
}

fn insert_attribute_call(
    id_defs: &mut IdNameMapBuilder,
    fn_body: &mut Vec<ExpandedStatement>,
    mut function: Id,
    intersection: bool,
    input_arguments: &[ast::VariableDeclaration<Id>],
    indirect_remap: Option<usize>,
) -> Result<Id, TranslateError> {
    fn input_arg_convert(var: &ast::VariableDeclaration<Id>) -> (Id, ast::Type, ast::StateSpace) {
        (var.name, var.type_.clone(), var.state_space)
    }
    let mut input_arguments: Vec<_> = input_arguments.iter().map(input_arg_convert).collect();
    if let Some(index) = indirect_remap {
        let constant_0_b64 = id_defs.register_intermediate(Some((
            ast::Type::Scalar(ast::ScalarType::B64),
            ast::StateSpace::Reg,
        )));
        fn_body.push(ExpandedStatement::Constant(ConstantDefinition {
            dst: constant_0_b64,
            typ: ast::ScalarType::B64,
            value: ast::ImmediateValue::U64(0),
        }));
        let is_null = id_defs.register_intermediate(Some((
            ast::Type::Scalar(ast::ScalarType::Pred),
            ast::StateSpace::Reg,
        )));
        fn_body.push(Statement::Instruction(ast::Instruction::Setp(
            ast::SetpData {
                typ: ast::ScalarType::B64,
                flush_to_zero: None,
                cmp_op: ast::SetpCompareOp::Eq,
            },
            ast::Arg4Setp {
                dst1: is_null,
                dst2: None,
                src1: function,
                src2: constant_0_b64,
            },
        )));
        let early_exit = id_defs.register_intermediate(None);
        let continue_execution = id_defs.register_intermediate(None);
        let constant_0_b32 = id_defs.register_intermediate(Some((
            ast::Type::Scalar(ast::ScalarType::B32),
            ast::StateSpace::Reg,
        )));
        fn_body.push(Statement::Conditional(BrachCondition {
            predicate: is_null,
            if_true: early_exit,
            if_false: continue_execution,
        }));
        fn_body.push(Statement::Label(early_exit));
        fn_body.push(ExpandedStatement::Constant(ConstantDefinition {
            dst: constant_0_b32,
            typ: ast::ScalarType::B32,
            value: ast::ImmediateValue::U64(0),
        }));
        fn_body.push(ExpandedStatement::RetValue(
            ast::RetData { uniform: false },
            vec![(constant_0_b32, ast::Type::Scalar(ast::ScalarType::B32))],
        ));
        fn_body.push(Statement::Label(continue_execution));
        input_arguments[index].0 = function;
        let fn_ptr = id_defs.register_intermediate(Some((
            ast::Type::Scalar(ast::ScalarType::B64),
            ast::StateSpace::Reg,
        )));
        fn_body.push(Statement::Instruction(ast::Instruction::Ld(
            ast::LdDetails {
                qualifier: ast::LdStQualifier::Weak,
                state_space: ast::StateSpace::Global,
                caching: ast::LdCacheOperator::Cached,
                typ: ast::Type::Scalar(ast::ScalarType::B64),
                non_coherent: false,
            },
            ast::Arg2Ld {
                dst: fn_ptr,
                src: function,
            },
        )));
        function = fn_ptr;
    }
    let return_value = id_defs.register_intermediate(Some((
        ast::Type::Scalar(ast::ScalarType::B32),
        ast::StateSpace::Reg,
    )));
    if intersection {
        let return_arguments = vec![];
        fn_body.push(Statement::Call(ResolvedCall {
            uniform: false,
            return_arguments,
            name: function,
            input_arguments,
            is_indirect: indirect_remap.is_some(),
        }));
    } else {
        let return_arguments = vec![(
            return_value,
            ast::Type::Scalar(ast::ScalarType::B32),
            ast::StateSpace::Reg,
        )];
        let call = ResolvedCall {
            uniform: false,
            return_arguments,
            name: function,
            input_arguments,
            is_indirect: indirect_remap.is_some(),
        };
        fn_body.push(Statement::Call(call));
        insert_call_with_exception_propagation(id_defs, fn_body, return_value, |_, _| {
            Ok(Vec::new())
        })?;
    };
    Ok(return_value)
}

fn insert_attribute_anyhit_check_rollback_jmp(
    id_defs: &mut IdNameMapBuilder,
    body: &mut Vec<ExpandedStatement>,
    anyhit_result: Id,
    return_anyhit: Id,
    rollback_on: u64,
) -> Id {
    let constant = id_defs.register_intermediate(Some((
        ast::Type::Scalar(ast::ScalarType::B32),
        ast::StateSpace::Reg,
    )));
    body.push(ExpandedStatement::Constant(ConstantDefinition {
        dst: constant,
        typ: ast::ScalarType::B32,
        value: ast::ImmediateValue::U64(rollback_on),
    }));
    let anyhit_is_eq = id_defs.register_intermediate(Some((
        ast::Type::Scalar(ast::ScalarType::Pred),
        ast::StateSpace::Reg,
    )));
    body.push(ExpandedStatement::Instruction(ast::Instruction::Setp(
        ast::SetpData {
            typ: ast::ScalarType::B32,
            flush_to_zero: None,
            cmp_op: ast::SetpCompareOp::Eq,
        },
        ast::Arg4Setp {
            dst1: anyhit_is_eq,
            dst2: None,
            src1: anyhit_result,
            src2: constant,
        },
    )));
    let rollback = id_defs.register_intermediate(None);
    body.push(ExpandedStatement::Conditional(BrachCondition {
        predicate: anyhit_is_eq,
        if_true: rollback,
        if_false: return_anyhit,
    }));
    body.push(ExpandedStatement::Label(rollback));
    rollback
}

fn insert_attribute_rollback<'input>(
    id_defs: &mut IdNameMapBuilder<'input>,
    result: &mut Vec<ExpandedStatement>,
    get_var_ptr_global: Id,
    attribute_block: Id,
    original_attribute_values: Vec<(Id, ast::Type, u32)>,
) {
    for (variable, type_, offset) in original_attribute_values.iter() {
        let var_ptr = insert_call_to_get_variable_pointer(
            id_defs,
            attribute_block,
            (get_var_ptr_global, ast::StateSpace::Global),
            result,
            *offset,
        );
        // TODO: use alignment
        result.push(ExpandedStatement::Instruction(ast::Instruction::St(
            ast::StData {
                qualifier: ast::LdStQualifier::Weak,
                state_space: ast::StateSpace::Global,
                caching: ast::StCacheOperator::Writeback,
                typ: type_.clone(),
            },
            ast::Arg2St {
                src1: var_ptr,
                src2: *variable,
            },
        )));
    }
}

fn expand_rt_functions<'input>(
    mut translation_module: TranslationModule<'input, ExpandedArgParams>,
    var_tracker: &FinalizedVariablesLayout<'input>,
) -> Result<TranslationModule<'input, ExpandedArgParams>, TranslateError> {
    let potential_intersection =
        get_function_id(&translation_module, "_rt_potential_intersection")?;
    let potential_intersection_new_args = [
        InjectedArgument::CurrentRay,
        InjectedArgument::IntersectionDistance,
    ];
    let report_intersection = get_function_id(&translation_module, "_rt_report_intersection")?;
    let report_intersection_new_args = [
        InjectedArgument::GlobalState,
        InjectedArgument::LaunchIndex,
        InjectedArgument::LaunchDim,
        InjectedArgument::CurrentRay,
        InjectedArgument::CurrentTime,
        InjectedArgument::Payload,
        InjectedArgument::VariablesBlock,
        InjectedArgument::AttributesBlock,
        InjectedArgument::TransformBlock,
        InjectedArgument::NewDistance,
        InjectedArgument::IntersectionResult,
        InjectedArgument::MaterialResult,
    ];
    let is_triangle_hit = get_function_id(&translation_module, "_rt_is_triangle_hit")?;
    let is_triangle_hit_new_args = [InjectedArgument::TriangleNormals];
    let is_triangle_hit_front_face =
        get_function_id(&translation_module, "_rt_is_triangle_hit_front_face")?;
    let is_triangle_hit_front_face_new_args = [
        InjectedArgument::CurrentRay,
        InjectedArgument::TriangleNormals,
    ];
    let is_triangle_hit_back_face =
        get_function_id(&translation_module, "_rt_is_triangle_hit_back_face")?;
    let is_triangle_hit_back_face_new_args = [
        InjectedArgument::CurrentRay,
        InjectedArgument::TriangleNormals,
    ];
    let rt_get_transform = get_function_id(&translation_module, "_rt_get_transform")?;
    let rt_get_transform_new_args = [InjectedArgument::TransformBlock];
    let rt_trace_mask_flags = get_function_id(&translation_module, "_rt_trace_mask_flags_64")?;
    let rt_trace_time_mask_flags =
        get_function_id(&translation_module, "_rt_trace_time_mask_flags_64")?;
    let rt_buffer_get_id_64 = get_function_id(&translation_module, "_rt_buffer_get_id_64")?;
    let rt_buffer_get_id_size_64 =
        get_function_id(&translation_module, "_rt_buffer_get_id_size_64")?;
    let rt_callable_program_from_id_64 =
        get_function_id(&translation_module, "_rt_callable_program_from_id_64")?;
    let rt_callable_program_from_id_v2_64 =
        get_function_id(&translation_module, "_rt_callable_program_from_id_v2_64")?;
    let rt_texture_get_f_id = get_function_id(&translation_module, "_rt_texture_get_f_id")?;
    let rt_texture_grad_load_or_request_f_id =
        get_function_id(&translation_module, "_rt_texture_grad_load_or_request_f_id")?;
    let rt_texture_lod_load_or_request_f_id =
        get_function_id(&translation_module, "_rt_texture_lod_load_or_request_f_id")?;
    let inject_globals_arg = [InjectedArgument::GlobalState];
    let injection_table = [
        (
            potential_intersection,
            &potential_intersection_new_args[..],
            true,
        ),
        (report_intersection, &report_intersection_new_args[..], true),
        (is_triangle_hit, &is_triangle_hit_new_args[..], false),
        (
            is_triangle_hit_front_face,
            &is_triangle_hit_front_face_new_args[..],
            false,
        ),
        (
            is_triangle_hit_back_face,
            &is_triangle_hit_back_face_new_args[..],
            false,
        ),
        (rt_get_transform, &rt_get_transform_new_args[..], false),
        (rt_trace_mask_flags, &inject_globals_arg[..], false),
        (rt_trace_time_mask_flags, &inject_globals_arg[..], false),
        (rt_buffer_get_id_64, &inject_globals_arg[..], false),
        (rt_buffer_get_id_size_64, &inject_globals_arg[..], false),
        (
            rt_callable_program_from_id_64,
            &inject_globals_arg[..],
            false,
        ),
        (
            rt_callable_program_from_id_v2_64,
            &inject_globals_arg[..],
            false,
        ),
        (rt_texture_get_f_id, &inject_globals_arg[..], false),
        (
            rt_texture_grad_load_or_request_f_id,
            &inject_globals_arg[..],
            false,
        ),
        (
            rt_texture_lod_load_or_request_f_id,
            &inject_globals_arg[..],
            false,
        ),
    ];
    for directive in translation_module.directives.iter_mut() {
        match directive {
            TranslationDirective::Method(method) => {
                let func_name = method.name;
                let extra_args = injection_table
                    .iter()
                    .find(|(name, _, _)| *name == func_name);
                if let Some((_, extra_args, add_potential_dist)) = extra_args {
                    append_rt_function_arguments_to_declaration(
                        &mut translation_module.id_defs,
                        &mut method.input_arguments,
                        extra_args,
                        *add_potential_dist,
                    );
                    continue;
                }
                if let Some(ref mut body) = method.body {
                    let injected_fn_args = var_tracker.injected_args.get(&func_name);
                    // Currently rollback wrapper does not emit information about injected args, so we skip
                    // TODO: make rollback wrapper emit injected args information and fail on None here
                    // TODO: replace match with unwrap_or_continue!(...)
                    let injected_fn_args = match injected_fn_args {
                        Some(a) => a,
                        None => continue,
                    };
                    *body = expand_rt_instructions(
                        &mut translation_module.id_defs,
                        mem::take(body),
                        &injection_table[..],
                        injected_fn_args,
                    )?;
                };
            }
            TranslationDirective::Variable(..) => {}
        }
    }
    Ok(translation_module)
}

fn get_function_id(
    translation_module: &TranslationModule<ExpandedArgParams>,
    name: &'static str,
) -> Result<Id, TranslateError> {
    translation_module
        .id_defs
        .globals
        .variables
        .get(name)
        .copied()
        .ok_or_else(TranslateError::unreachable)
}

fn replace_attribute_functions<'input>(
    mut translation_module: TranslationModule<'input, ExpandedArgParams>,
    var_tracker: &FinalizedVariablesLayout<'input>,
) -> Result<TranslationModule<'input, ExpandedArgParams>, TranslateError> {
    let get_triangle_barycentrics = translation_module
        .id_defs
        .globals
        .variables
        .get("_rt_get_triangle_barycentrics")
        .ok_or_else(TranslateError::unreachable)?;
    let get_triangle_barycentrics_marker = InjectedArgument::TriangleBarycentrics;
    let get_primitive_index = translation_module
        .id_defs
        .globals
        .variables
        .get("_rt_get_primitive_index")
        .ok_or_else(TranslateError::unreachable)?;
    let get_primitive_index_marker = InjectedArgument::PrimitiveIndex;
    let replacement_candidates = [
        (*get_triangle_barycentrics, get_triangle_barycentrics_marker),
        (*get_primitive_index, get_primitive_index_marker),
    ];
    for directive in translation_module.directives.iter_mut() {
        match directive {
            TranslationDirective::Method(method) => {
                let func_name = if method.is_kernel {
                    continue;
                } else {
                    method.name
                };
                let function_injected_args = match var_tracker.injected_args.get(&func_name) {
                    Some(args) => args,
                    None => continue,
                };
                for statement in method.body.iter_mut().flatten() {
                    match statement {
                        Statement::Call(call) => {
                            if let Some(replacement_statement) = replace_attribute_function_call(
                                call,
                                function_injected_args,
                                replacement_candidates.iter().copied(),
                            )? {
                                *statement = replacement_statement;
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    Ok(translation_module)
}

fn replace_attribute_function_call(
    call: &mut ResolvedCall<ExpandedArgParams>,
    injected_args: &InjectedArguments,
    candidates: impl Iterator<Item = (Id, InjectedArgument)>,
) -> Result<Option<ExpandedStatement>, TranslateError> {
    for (function, injected_kind) in candidates {
        if call.name != function {
            continue;
        }
        let method_arg = injected_args
            .get(injected_kind)
            .ok_or_else(TranslateError::unreachable)?;
        let (typ, space) = injected_kind.get_type_space();
        if space != ast::StateSpace::Reg {
            return Err(TranslateError::unreachable());
        }
        if call.return_arguments.len() == 1 {
            return Ok(Some(Statement::Instruction(ast::Instruction::Mov(
                ast::MovDetails {
                    typ: typ.clone(),
                    dst_width: 0,
                    src_width: 0,
                    relaxed_src2_conv: false,
                },
                ast::Arg2Mov {
                    dst: call.return_arguments[0].0,
                    src: method_arg,
                },
            ))));
        } else {
            let (scalar_type, vector_len) = match typ {
                ast::Type::Vector(t, l) => (t, l),
                _ => return Err(TranslateError::unreachable()),
            };
            if call.return_arguments.len() != vector_len as usize {
                return Err(TranslateError::unreachable());
            }
            return Ok(Some(Statement::RepackVector(RepackVectorDetails {
                is_extract: true,
                typ: scalar_type,
                packed: method_arg,
                unpacked: call.return_arguments.iter().map(|(id, _, _)| *id).collect(),
                non_default_implicit_conversion: None,
            })));
        }
    }
    Ok(None)
}

fn expand_rt_instructions(
    id_defs: &mut IdNameMapBuilder,
    method_body: Vec<ExpandedStatement>,
    injection_table: &[(Id, &[InjectedArgument], bool)],
    injected_fn_args: &InjectedArguments,
) -> Result<Vec<ExpandedStatement>, TranslateError> {
    let mut result = Vec::with_capacity(method_body.len());
    let potential_distance_var = id_defs.register_variable_def(
        None,
        ast::Type::Scalar(ast::ScalarType::F32),
        ast::StateSpace::Local,
        Some(ast::Initializer::Constant(ast::ImmediateValue::F32(0f32))),
    );
    let potential_distance = potential_distance_var.name;
    result.push(Statement::Variable(potential_distance_var));
    for mut statement in method_body {
        if let Statement::Call(ref mut call) = statement {
            if let Some((_, new_args, inject_distance)) = injection_table
                .iter()
                .find(|(name, _, _)| *name == call.name)
            {
                append_intersection_arguments_to_call(
                    &mut call.input_arguments,
                    new_args,
                    injected_fn_args,
                    if *inject_distance {
                        Some(potential_distance)
                    } else {
                        None
                    },
                )?;
            }
        }
        result.push(statement);
    }
    Ok(result)
}

fn append_rt_function_arguments_to_declaration(
    id_defs: &mut IdNameMapBuilder,
    input_arguments: &mut Vec<ast::VariableDeclaration<Id>>,
    new_args: &[InjectedArgument],
    add_potential_distance: bool,
) {
    for arg in new_args {
        let (type_, state_space) = arg.get_type_space();
        let name = id_defs.register_intermediate(Some((type_.clone(), state_space)));
        input_arguments.push(ast::VariableDeclaration {
            type_,
            state_space,
            name,
            align: None,
        });
    }
    if !add_potential_distance {
        return;
    }
    let name = id_defs.register_intermediate(Some((
        ast::Type::Scalar(ast::ScalarType::F32),
        ast::StateSpace::Local,
    )));
    input_arguments.push(ast::VariableDeclaration {
        type_: ast::Type::Scalar(ast::ScalarType::F32),
        state_space: ast::StateSpace::Local,
        name,
        align: None,
    })
}

fn append_intersection_arguments_to_call(
    input_arguments: &mut Vec<(Id, ast::Type, ast::StateSpace)>,
    new_args: &[InjectedArgument],
    fn_args: &InjectedArguments,
    maybe_potential_distance: Option<Id>,
) -> Result<(), TranslateError> {
    for arg in new_args {
        let name = fn_args.get(*arg).ok_or_else(TranslateError::unreachable)?;
        let (type_, state_space) = arg.get_type_space();
        input_arguments.push((name, type_, state_space));
    }
    if let Some(potential_distance) = maybe_potential_distance {
        input_arguments.push((
            potential_distance,
            ast::Type::Scalar(ast::ScalarType::F32),
            ast::StateSpace::Local,
        ));
    }
    Ok(())
}

// Optix and HIP RT represent bounding boxes slightly differently
// Optix uses (xmin, ymin, zmin), (xmax, ymax, zmax)
// HIP RT uses (xmin, ymin, zmin, unused), (xmax, ymax, zmax, unused)
// This pass converts all store from OptiX format to HIP RT
// TODO: This pass is very brittle, have a fallback conversion if it does not succeed
fn convert_arguments_in_bounding_box_kernel<'input>(
    mut translation_module: TranslationModule<'input, NormalizedArgParams>,
    raytracing_state: &RaytracingTranslationState,
) -> Result<TranslationModule<'input, NormalizedArgParams>, TranslateError> {
    if raytracing_state.entry_point_kind != Some(RaytracingEntryPointKind::BoundingBox) {
        return Ok(translation_module);
    }
    let entry_point_id = raytracing_state.entry_point_id.unwrap();
    for directive in translation_module.directives.iter_mut() {
        match directive {
            TranslationDirective::Method(method) => {
                if method.name != entry_point_id {
                    continue;
                }
                let variable_id = method.input_arguments[1].name;
                let mut lds = FxHashSet::default();
                for statement in method.body.iter().flatten() {
                    match statement {
                        Statement::Instruction(ast::Instruction::Ld(
                            ast::LdDetails {
                                state_space: ast::StateSpace::Param,
                                typ,
                                ..
                            },
                            ast::Arg2Ld {
                                dst: ast::Operand::Reg(dst),
                                src: ast::Operand::Reg(src_reg),
                            },
                        )) if *src_reg == variable_id
                            && typ.layout().size() == mem::size_of::<*const c_void>() =>
                        {
                            lds.insert(*dst);
                        }
                        _ => {}
                    }
                }
                if lds.len() == 0 {
                    return Err(TranslateError::unexpected_pattern());
                }
                let mut cvtas = FxHashSet::default();
                for statement in method.body.iter().flatten() {
                    match statement {
                        Statement::Instruction(ast::Instruction::Cvta(
                            ast::CvtaDetails {
                                to: ast::StateSpace::Global,
                                ..
                            },
                            ast::Arg2 {
                                dst: ast::Operand::Reg(dst),
                                src: ast::Operand::Reg(src),
                            },
                        )) if lds.contains(src) => {
                            cvtas.insert(*dst);
                        }
                        _ => {}
                    }
                }
                if cvtas.len() == 0 {
                    return Err(TranslateError::unexpected_pattern());
                }
                for statement in method.body.iter_mut().flatten() {
                    match statement {
                        Statement::Instruction(ast::Instruction::St(
                            _,
                            ast::Arg2St {
                                src1: ast::Operand::RegOffset(src_reg, offset),
                                ..
                            },
                        )) if cvtas.contains(src_reg) => {
                            if *offset >= (mem::size_of::<f32>() * 3) as i64 {
                                *offset = *offset + mem::size_of::<f32>() as i64;
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    Ok(translation_module)
}

// We do following transformations:
// * Make all methods with bodies except entry point private
// * Turn entry point from kernel to a function
// * Turn existing kernel .param arguments into .reg arguments
pub(crate) fn transform_entry_point<'ast, P: ast::ArgParams<Id = Id>>(
    mut translation_module: TranslationModule<'ast, P>,
    translation_state: &mut RaytracingTranslationState,
) -> Result<TranslationModule<'ast, P>, TranslateError> {
    let mut suffixed_entry_point = translation_state.entry_point_str.to_string();
    suffixed_entry_point.push('(');
    for directive in translation_module.directives.iter_mut() {
        if let TranslationDirective::Method(method) = directive {
            if let Some(kernel_name) = is_raytracing_entry_point(
                &method.source_name,
                translation_state.entry_point_str,
                &suffixed_entry_point,
            )? {
                let kind = if !method.is_kernel {
                    RaytracingEntryPointKind::Callable
                } else if method.input_arguments.len() == 2 {
                    RaytracingEntryPointKind::BoundingBox
                } else if method.input_arguments.len() == 1 {
                    RaytracingEntryPointKind::Intersection
                } else {
                    RaytracingEntryPointKind::Unknown
                };
                method.is_kernel = false;
                method.special_raytracing_linking = true;
                translation_state.kernel_name = kernel_name.to_string();
                translation_state.entry_point_id = Some(method.name);
                translation_state.entry_point_kind = Some(kind);
            }
        }
    }
    if let None = translation_state.entry_point_id {
        return Err(TranslateError::unexpected_pattern());
    }
    Ok(translation_module)
}

// We do following transformations:
// * Inject implicit OptiX parameters into entry point arguments
// * Convert Optix variables with builtin-semantic to movs from entry point arguments
// * Convert Optix variables with no builtin-semantic to loads from variable block
// * Convert Optix attribute variables to load from attribute block
// * Remove all globals (variables and methods)
// TODO: support non-2D launch indexes
// TODO: propagate RT variables to subfunctions
fn convert_global_variables<'a, 'ast>(
    mut translation_module: TranslationModule<'ast, ExpandedArgParams>,
    raytracing_state: &mut RaytracingTranslationState<'a, 'ast>,
    entry_point: Id,
) -> Result<
    (
        TranslationModule<'ast, ExpandedArgParams>,
        FinalizedVariablesLayout<'ast>,
    ),
    TranslateError,
> {
    let kind = raytracing_state
        .entry_point_kind
        .ok_or_else(TranslateError::unreachable)?;
    let variables_layout =
        collect_variable_and_attribute_declarations(&translation_module, raytracing_state)?;
    let id_defs = &mut translation_module.id_defs;
    let mut directives = Vec::new();
    let (get_var_pointer_global_decl, get_variable_pointer_global) = create_func_declaration(
        id_defs,
        false,
        &[ast::Type::Pointer(
            ast::ScalarType::B8,
            ast::StateSpace::Global,
        )],
        "__zluda_rt_ptx_impl__get_variable_pointer_global",
        &[
            ast::Type::Pointer(ast::ScalarType::B8, ast::StateSpace::Global),
            ast::Type::Scalar(ast::ScalarType::B32),
        ],
    );
    let (get_var_pointer_shared_decl, get_variable_pointer_shared) = create_func_declaration(
        id_defs,
        false,
        &[ast::Type::Pointer(
            ast::ScalarType::B8,
            ast::StateSpace::Shared,
        )],
        "__zluda_rt_ptx_impl__get_variable_pointer_shared",
        &[
            ast::Type::Pointer(ast::ScalarType::B8, ast::StateSpace::Shared),
            ast::Type::Scalar(ast::ScalarType::B32),
        ],
    );
    directives.push(get_var_pointer_global_decl);
    directives.push(get_var_pointer_shared_decl);
    let (directives, mut layout_and_args) =
        remove_unused_globals_and_inject_arguments_into_entry_point(
            id_defs,
            mem::take(&mut translation_module.directives),
            directives,
            entry_point,
            kind,
            variables_layout,
        )?;
    translation_module.directives = propagate_injected_arguments_to_callees(
        id_defs,
        directives,
        entry_point,
        &raytracing_state.reachable_user_functions,
        &mut layout_and_args,
    )?;
    translation_module.directives = convert_optix_builtin_variable_and_attribute_access(
        id_defs,
        mem::take(&mut translation_module.directives),
        &layout_and_args,
        (get_variable_pointer_global, get_variable_pointer_shared),
    )?;
    translation_module.directives = fixup_variables_with_empty_semantic(
        id_defs,
        mem::take(&mut translation_module.directives),
        &layout_and_args,
        get_variable_pointer_global,
    )?;
    Ok((translation_module, layout_and_args))
}

fn propagate_injected_arguments_to_callees<'ast>(
    id_defs: &mut IdNameMapBuilder<'ast>,
    mut directives: Vec<TranslationDirective<'ast, ExpandedArgParams>>,
    entry_point_id: Id,
    reachable_functions: &FxHashSet<Id>,
    var_tracker: &mut FinalizedVariablesLayout<'ast>,
) -> Result<Vec<TranslationDirective<'ast, ExpandedArgParams>>, TranslateError> {
    let entry_point_method = get_function_by_id(&directives, entry_point_id)?;
    let injected_args = get_injected_args(
        entry_point_id,
        &entry_point_method,
        &var_tracker.injected_args,
    )?;
    propagate_injected_args(
        id_defs,
        &mut directives,
        entry_point_id,
        reachable_functions,
        var_tracker,
        injected_args,
    )?;
    Ok(directives)
}

// We've removed all unreachable functions so everything at this point is reachable
fn propagate_injected_args<'input>(
    id_defs: &mut IdNameMapBuilder,
    directives: &mut Vec<TranslationDirective<ExpandedArgParams>>,
    entry_point: Id,
    reachable_functions: &FxHashSet<Id>,
    var_tracker: &mut FinalizedVariablesLayout<'input>,
    injected_args: Vec<InjectedArgument>,
) -> Result<(), TranslateError> {
    for directive in directives {
        if let TranslationDirective::Method(method) = directive {
            let func_name = method.name;
            let func_injected_args = if func_name == entry_point {
                var_tracker
                    .injected_args
                    .get(&func_name)
                    .ok_or_else(TranslateError::unreachable)?
            } else {
                let mut func_injected_args = InjectedArguments::new();
                for arg in injected_args.iter().copied() {
                    let (type_, state_space) = arg.get_type_space();
                    let name = id_defs.register_intermediate(Some((type_.clone(), state_space)));
                    method.input_arguments.push(ast::VariableDeclaration {
                        align: None,
                        type_,
                        state_space,
                        name,
                    });
                    func_injected_args.set(arg, name);
                }
                if method.body.is_some() {
                    match var_tracker.injected_args.entry(func_name) {
                        std::collections::hash_map::Entry::Occupied(_) => {
                            return Err(TranslateError::unreachable());
                        }
                        std::collections::hash_map::Entry::Vacant(entry) => {
                            entry.insert(func_injected_args)
                        }
                    }
                } else {
                    continue;
                }
            };
            for statement in method.body.iter_mut().flatten() {
                if let Statement::Call(call) = statement {
                    if reachable_functions.contains(&call.name) {
                        for arg_kind in injected_args.iter() {
                            let (type_, space) = arg_kind.get_type_space();
                            let name = func_injected_args
                                .get(*arg_kind)
                                .ok_or_else(TranslateError::unreachable)?;
                            call.input_arguments.push((name, type_, space));
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn get_function_by_id<'a, 'ast>(
    directives: &'a [TranslationDirective<'ast, ExpandedArgParams>],
    fn_name: Id,
) -> Result<&'a TranslationMethod<'ast, ExpandedArgParams>, TranslateError> {
    directives
        .iter()
        .find_map(|directive| {
            if let TranslationDirective::Method(method) = directive {
                if method.name == fn_name {
                    return Some(method);
                }
            }
            None
        })
        .ok_or_else(TranslateError::unreachable)
}

fn get_injected_args<'a, 'ast>(
    entry_point_id: Id,
    entry_point: &'a TranslationMethod<'ast, ExpandedArgParams>,
    injected_args: &FxHashMap<Id, InjectedArguments>,
) -> Result<Vec<InjectedArgument>, TranslateError> {
    let entry_injected = injected_args
        .get(&entry_point_id)
        .ok_or_else(TranslateError::unreachable)?;
    Ok(entry_point
        .input_arguments
        .iter()
        .filter_map(|arg| entry_injected.find_by_id(arg.name))
        .collect::<Vec<_>>())
}

fn remove_unused_globals_and_inject_arguments_into_entry_point<'a, 'ast>(
    id_defs: &mut IdNameMapBuilder<'ast>,
    old_directives: Vec<TranslationDirective<'ast, ExpandedArgParams>>,
    mut new_directives: Vec<TranslationDirective<'ast, ExpandedArgParams>>,
    entry_point: Id,
    kind: RaytracingEntryPointKind,
    variables: VariablesLayout<'ast>,
) -> Result<
    (
        Vec<TranslationDirective<'ast, ExpandedArgParams>>,
        FinalizedVariablesLayout<'ast>,
    ),
    TranslateError,
> {
    let mut variables_with_args = FinalizedVariablesLayout {
        variables,
        injected_args: FxHashMap::default(),
    };
    for directive in old_directives {
        match directive {
            TranslationDirective::Variable(_, _, variable) => {
                if let Some(name) = id_defs.globals.reverse_variables.get(&variable.name) {
                    if let Some(demangled_name) = demangle(name) {
                        // Those variables are somehow used by exception programs
                        // TODO: use this correctly
                        if demangled_name.starts_with("rti_internal_register::") {
                            new_directives.push(TranslationDirective::Variable(
                                ast::LinkingDirective::None,
                                None,
                                variable,
                            ));
                            continue;
                        } else if demangled_name.starts_with("rti_internal") {
                            continue;
                        }
                    }
                }
                // String constants are used by exception handling
                if variable.initializer.is_some() {
                    new_directives.push(TranslationDirective::Variable(
                        ast::LinkingDirective::None,
                        None,
                        variable,
                    ));
                }
            }
            TranslationDirective::Method(mut method) => {
                if method.name != entry_point {
                    new_directives.push(TranslationDirective::Method(method));
                    continue;
                }
                let mut injected_args = InjectedArguments::new();
                injected_args.set(
                    InjectedArgument::GlobalState,
                    inject_argument(
                        InjectedArgument::GlobalState.get_type_space(),
                        id_defs,
                        &mut method,
                    ),
                );
                if kind == RaytracingEntryPointKind::Unknown {
                    injected_args.set(
                        InjectedArgument::PrimitiveIndex,
                        inject_argument(
                            InjectedArgument::PrimitiveIndex.get_type_space(),
                            id_defs,
                            &mut method,
                        ),
                    );
                } else if kind == RaytracingEntryPointKind::BoundingBox
                    || kind == RaytracingEntryPointKind::Intersection
                {
                    injected_args.set(
                        InjectedArgument::PrimitiveIndex,
                        method.input_arguments[0].name,
                    );
                }
                variables_with_args.remap_fn_arguments(
                    kind,
                    &mut injected_args,
                    |type_, state_space| {
                        let var_name =
                            id_defs.register_intermediate(Some((type_.clone(), state_space)));
                        method.input_arguments.push(ast::VariableDeclaration {
                            type_: type_,
                            state_space,
                            name: var_name,
                            align: None,
                        });
                        var_name
                    },
                );
                if kind == RaytracingEntryPointKind::Unknown {
                    injected_args.set(
                        InjectedArgument::TriangleBarycentrics,
                        inject_argument(
                            InjectedArgument::TriangleBarycentrics.get_type_space(),
                            id_defs,
                            &mut method,
                        ),
                    );
                    injected_args.set(
                        InjectedArgument::TriangleNormals,
                        inject_argument(
                            InjectedArgument::TriangleNormals.get_type_space(),
                            id_defs,
                            &mut method,
                        ),
                    );
                }
                injected_args.set(
                    InjectedArgument::VariablesBlock,
                    inject_argument(
                        InjectedArgument::VariablesBlock.get_type_space(),
                        id_defs,
                        &mut method,
                    ),
                );
                if kind != RaytracingEntryPointKind::BoundingBox
                    && kind != RaytracingEntryPointKind::Callable
                {
                    injected_args.set(
                        InjectedArgument::AttributesBlock,
                        inject_argument(
                            InjectedArgument::AttributesBlock.get_type_space(),
                            id_defs,
                            &mut method,
                        ),
                    );
                    injected_args.set(
                        InjectedArgument::TransformBlock,
                        inject_argument(
                            InjectedArgument::TransformBlock.get_type_space(),
                            id_defs,
                            &mut method,
                        ),
                    );
                }
                if kind == RaytracingEntryPointKind::Intersection {
                    injected_args.set(
                        InjectedArgument::NewDistance,
                        inject_argument(
                            InjectedArgument::NewDistance.get_type_space(),
                            id_defs,
                            &mut method,
                        ),
                    );
                    injected_args.set(
                        InjectedArgument::IntersectionResult,
                        inject_argument(
                            InjectedArgument::IntersectionResult.get_type_space(),
                            id_defs,
                            &mut method,
                        ),
                    );
                    injected_args.set(
                        InjectedArgument::MaterialResult,
                        inject_argument(
                            InjectedArgument::MaterialResult.get_type_space(),
                            id_defs,
                            &mut method,
                        ),
                    );
                }
                variables_with_args
                    .injected_args
                    .insert(entry_point, injected_args);
                new_directives.push(TranslationDirective::Method(method))
            }
        }
    }
    Ok((new_directives, variables_with_args))
}

fn inject_argument<'input, P: ast::ArgParams<Id = Id>>(
    (type_, state_space): (ast::Type, ast::StateSpace),
    id_defs: &mut IdNameMapBuilder,
    func_decl: &mut TranslationMethod<'input, P>,
) -> Id {
    let variable_block = id_defs.register_intermediate(Some((type_.clone(), state_space)));
    func_decl.input_arguments.push(ast::VariableDeclaration {
        state_space,
        type_: type_,
        name: variable_block,
        align: None,
    });
    variable_block
}

fn fixup_variables_with_empty_semantic<'a, 'input>(
    id_defs: &mut IdNameMapBuilder<'input>,
    directives: Vec<TranslationDirective<'input, ExpandedArgParams>>,
    var_tracker: &FinalizedVariablesLayout<'input>,
    get_variable_pointer: Id,
) -> Result<Vec<TranslationDirective<'input, ExpandedArgParams>>, TranslateError> {
    let mut result = Vec::with_capacity(directives.len());
    for directive in directives {
        let new_directive = match directive {
            TranslationDirective::Method(method) => fixup_variable_with_empty_semantic_method(
                id_defs,
                var_tracker,
                get_variable_pointer,
                method,
            )?,
            TranslationDirective::Variable(linking, compiled_name, var) => {
                TranslationDirective::Variable(linking, compiled_name, var)
            }
        };
        result.push(new_directive);
    }
    Ok(result)
}

fn fixup_variable_with_empty_semantic_method<'a, 'input>(
    id_defs: &mut IdNameMapBuilder<'input>,
    var_tracker: &FinalizedVariablesLayout<'input>,
    get_variable_pointer: Id,
    mut method: TranslationMethod<'input, ExpandedArgParams>,
) -> Result<TranslationDirective<'input, ExpandedArgParams>, TranslateError> {
    let variables_block = if let Some(injected_vars) = var_tracker.injected_args.get(&method.name) {
        injected_vars
            .get(InjectedArgument::VariablesBlock)
            .ok_or_else(TranslateError::unreachable)?
    } else {
        return Ok(TranslationDirective::Method(method));
    };
    method.body = method
        .body
        .map(|body| {
            let mut result = Vec::with_capacity(body.len());
            for statement in body {
                let new_statement = match statement {
                    Statement::Call(call) => fixup_variable_with_empty_semantic_statement(
                        id_defs,
                        var_tracker,
                        get_variable_pointer,
                        variables_block,
                        &mut result,
                        call,
                    )?,
                    Statement::Instruction(inst) => fixup_variable_with_empty_semantic_statement(
                        id_defs,
                        var_tracker,
                        get_variable_pointer,
                        variables_block,
                        &mut result,
                        inst,
                    )?,
                    Statement::PtrAccess(access) => fixup_variable_with_empty_semantic_statement(
                        id_defs,
                        var_tracker,
                        get_variable_pointer,
                        variables_block,
                        &mut result,
                        access,
                    )?,
                    Statement::RepackVector(repack) => {
                        fixup_variable_with_empty_semantic_statement(
                            id_defs,
                            var_tracker,
                            get_variable_pointer,
                            variables_block,
                            &mut result,
                            repack,
                        )?
                    }
                    Statement::MadC(madc) => fixup_variable_with_empty_semantic_statement(
                        id_defs,
                        var_tracker,
                        get_variable_pointer,
                        variables_block,
                        &mut result,
                        madc,
                    )?,
                    Statement::MadCC(madcc) => fixup_variable_with_empty_semantic_statement(
                        id_defs,
                        var_tracker,
                        get_variable_pointer,
                        variables_block,
                        &mut result,
                        madcc,
                    )?,
                    Statement::AddC(details, arg) => fixup_variable_with_empty_semantic_statement(
                        id_defs,
                        var_tracker,
                        get_variable_pointer,
                        variables_block,
                        &mut result,
                        translate::VisitAddC(details, arg),
                    )?,
                    Statement::AddCC(type_, arg) => fixup_variable_with_empty_semantic_statement(
                        id_defs,
                        var_tracker,
                        get_variable_pointer,
                        variables_block,
                        &mut result,
                        translate::VisitAddCC(type_, arg),
                    )?,
                    Statement::SubC(details, arg) => fixup_variable_with_empty_semantic_statement(
                        id_defs,
                        var_tracker,
                        get_variable_pointer,
                        variables_block,
                        &mut result,
                        translate::VisitSubC(details, arg),
                    )?,
                    Statement::SubCC(type_, arg) => fixup_variable_with_empty_semantic_statement(
                        id_defs,
                        var_tracker,
                        get_variable_pointer,
                        variables_block,
                        &mut result,
                        translate::VisitSubCC(type_, arg),
                    )?,
                    s @ Statement::Conditional(_)
                    | s @ Statement::Conversion(_)
                    | s @ Statement::Label(_)
                    | s @ Statement::Constant(_)
                    | s @ Statement::Variable(_)
                    | s @ Statement::LoadVar(..)
                    | s @ Statement::StoreVar(..)
                    | s @ Statement::RetValue(..)
                    | s @ Statement::AsmVolatile { .. }
                    | s @ Statement::FunctionPointer(..) => s,
                };
                result.push(new_statement);
            }
            Ok(result)
        })
        .transpose()?;
    Ok(TranslationDirective::Method(method))
}

fn fixup_variable_with_empty_semantic_statement<'input>(
    id_def: &mut IdNameMapBuilder,
    var_tracker: &FinalizedVariablesLayout<'input>,
    get_variable_pointer: Id,
    variables_block: Id,
    result: &mut Vec<ExpandedStatement>,
    stmt: impl Visitable<ExpandedArgParams, ExpandedArgParams>,
) -> Result<ExpandedStatement, TranslateError> {
    stmt.visit(&mut |arg_desc: ArgumentDescriptor<Id>,
                     _type: Option<(&ast::Type, ast::StateSpace)>| {
        if let Some(offset) = var_tracker.empty_semantic_variable_offset(arg_desc.op)? {
            Ok(insert_call_to_get_variable_pointer(
                id_def,
                variables_block,
                (get_variable_pointer, ast::StateSpace::Global),
                result,
                offset,
            ))
        } else {
            Ok(arg_desc.op)
        }
    })
}

fn insert_call_to_get_variable_pointer(
    id_def: &mut IdNameMapBuilder,
    variable_block: Id,
    (get_variable_pointer_name, get_variable_pointer_space): (Id, ast::StateSpace),
    result: &mut Vec<ExpandedStatement>,
    offset: u32,
) -> Id {
    let return_value = id_def.register_intermediate(Some((
        ast::Type::Pointer(ast::ScalarType::B8, get_variable_pointer_space),
        ast::StateSpace::Reg,
    )));
    let offset_constant = id_def.register_intermediate(Some((
        ast::Type::Scalar(ast::ScalarType::B32),
        ast::StateSpace::Reg,
    )));
    result.push(ExpandedStatement::Constant(ConstantDefinition {
        dst: offset_constant,
        typ: ast::ScalarType::B32,
        value: ast::ImmediateValue::U64(offset as u64),
    }));
    result.push(ExpandedStatement::Call(ResolvedCall {
        uniform: false,
        return_arguments: vec![(
            return_value,
            ast::Type::Pointer(ast::ScalarType::B8, get_variable_pointer_space),
            ast::StateSpace::Reg,
        )],
        name: get_variable_pointer_name,
        input_arguments: vec![
            (
                variable_block,
                ast::Type::Pointer(ast::ScalarType::B8, get_variable_pointer_space),
                ast::StateSpace::Reg,
            ),
            (
                offset_constant,
                ast::Type::Scalar(ast::ScalarType::B32),
                ast::StateSpace::Reg,
            ),
        ],
        is_indirect: false,
    }));
    return_value
}

fn convert_optix_builtin_variable_and_attribute_access<'input>(
    id_defs: &mut IdNameMapBuilder<'input>,
    mut directives: Vec<TranslationDirective<'input, ExpandedArgParams>>,
    var_tracker: &FinalizedVariablesLayout<'input>,
    get_variable_pointer: (Id, Id),
) -> Result<Vec<TranslationDirective<'input, ExpandedArgParams>>, TranslateError> {
    for directive in directives.iter_mut() {
        if let TranslationDirective::Method(method) = directive {
            let func_name = method.name;
            method.body = method
                .body
                .take()
                .map(|statements| {
                    convert_optix_builtin_variable_and_attribute_access_single_function(
                        id_defs,
                        statements,
                        func_name,
                        var_tracker,
                        get_variable_pointer,
                    )
                })
                .transpose()?;
        }
    }
    Ok(directives)
}

fn convert_optix_builtin_variable_and_attribute_access_single_function<'input>(
    id_defs: &mut IdNameMapBuilder<'input>,
    statements: Vec<ExpandedStatement>,
    fn_name: Id,
    var_tracker: &FinalizedVariablesLayout<'input>,
    get_variable_pointer: (Id, Id),
) -> Result<Vec<ExpandedStatement>, TranslateError> {
    let mut result = Vec::with_capacity(statements.len());
    let mut secondary_global_remap = FxHashMap::default();
    let mut ptr_mov_dsts = FxHashMap::default();
    let mut ptr_mov_variables = FxHashMap::default();
    let mut cvta_src_remap = FxHashMap::default();
    for statement in statements {
        match statement {
            Statement::Instruction(ast::Instruction::Mov(
                ast::MovDetails {
                    typ: ast::Type::Scalar(ast::ScalarType::U64),
                    ..
                },
                ast::Arg2Mov { dst, src },
            ))
            | Statement::Instruction(ast::Instruction::Mov(
                ast::MovDetails {
                    typ: ast::Type::Scalar(ast::ScalarType::B64),
                    ..
                },
                ast::Arg2Mov { dst, src },
            )) => {
                if let Some(remapping) = var_tracker.get_remapped_state_space(fn_name, src)? {
                    let (new_src, override_space) =
                        get_new_id_space(remapping, id_defs, &mut result, get_variable_pointer)?;
                    result.push(Statement::Instruction(ast::Instruction::Mov(
                        ast::MovDetails {
                            typ: ast::Type::Scalar(ast::ScalarType::U64),
                            dst_width: 0,
                            src_width: 0,
                            relaxed_src2_conv: false,
                        },
                        ast::Arg2Mov {
                            dst: dst,
                            src: new_src,
                        },
                    )));
                    ptr_mov_dsts.insert(dst, override_space);
                } else {
                    result.push(statement);
                }
            }
            Statement::StoreVar(StoreVarDetails {
                arg: ast::Arg2St { src1, src2 },
                ..
            }) => {
                if let Some(new_space) = ptr_mov_dsts.get(&src2) {
                    ptr_mov_variables.insert(src1, *new_space);
                }
                result.push(statement);
            }
            Statement::LoadVar(LoadVarDetails {
                arg: ast::Arg2 { dst, src },
                ..
            }) => {
                if let Some(new_space) = ptr_mov_variables.get(&src) {
                    cvta_src_remap.insert(dst, *new_space);
                }
                result.push(statement);
            }
            Statement::Instruction(ast::Instruction::Cvta(
                ast::CvtaDetails {
                    to: ast::StateSpace::Generic,
                    size,
                    ..
                },
                ast::Arg2 { dst, src },
            )) => {
                if let Some(new_space) = cvta_src_remap.get(&src) {
                    result.push(Statement::Instruction(ast::Instruction::Cvta(
                        ast::CvtaDetails {
                            from: *new_space,
                            to: ast::StateSpace::Generic,
                            size,
                        },
                        ast::Arg2 { dst: dst, src: src },
                    )));
                } else {
                    result.push(statement);
                }
            }
            Statement::Instruction(ast::Instruction::St(
                ast::StData { ref typ, .. },
                ast::Arg2St { src1, src2 },
            )) => {
                if let Some(remapping) = var_tracker
                    .get_remapped_state_space(fn_name, src1)?
                    .or_else(|| secondary_global_remap.get(&src1).copied())
                {
                    let (src1, override_space) =
                        get_new_id_space(remapping, id_defs, &mut result, get_variable_pointer)?;
                    if override_space == ast::StateSpace::Reg {
                        return Err(TranslateError::unexpected_pattern());
                    }
                    result.push(Statement::Instruction(ast::Instruction::St(
                        ast::StData {
                            qualifier: ast::LdStQualifier::Weak,
                            state_space: override_space,
                            caching: ast::StCacheOperator::Writeback,
                            typ: typ.clone(),
                        },
                        ast::Arg2St { src1, src2: src2 },
                    )));
                } else {
                    result.push(statement);
                }
            }
            Statement::Instruction(ast::Instruction::Ld(
                ast::LdDetails { ref typ, .. },
                ast::Arg2Ld { dst, src },
            )) => {
                if let Some(remapping) = var_tracker
                    .get_remapped_state_space(fn_name, src)?
                    .or_else(|| secondary_global_remap.get(&src).copied())
                {
                    let (src, override_space) =
                        get_new_id_space(remapping, id_defs, &mut result, get_variable_pointer)?;
                    result.push(Statement::Instruction(ast::Instruction::Ld(
                        ast::LdDetails {
                            typ: typ.clone(),
                            qualifier: ast::LdStQualifier::Weak,
                            state_space: override_space,
                            caching: ast::LdCacheOperator::Cached,
                            non_coherent: false,
                        },
                        ast::Arg2Ld { dst: dst, src },
                    )));
                } else {
                    result.push(statement);
                }
            }
            Statement::PtrAccess(PtrAccess {
                ref underlying_type,
                dst,
                ptr_src,
                offset_src,
                ..
            }) => {
                if let Some(remapping) = var_tracker.get_remapped_state_space(fn_name, ptr_src)? {
                    let (override_id, override_space) =
                        get_new_id_space(remapping, id_defs, &mut result, get_variable_pointer)?;
                    let new_dst = id_defs
                        .register_intermediate(Some((underlying_type.clone(), override_space)));
                    result.push(Statement::PtrAccess(PtrAccess {
                        underlying_type: underlying_type.clone(),
                        state_space: override_space,
                        dst: new_dst,
                        ptr_src: override_id,
                        offset_src: offset_src,
                    }));
                    secondary_global_remap.insert(dst, StateSpaceRemapping::ToVariable(new_dst));
                } else {
                    result.push(statement);
                }
            }
            Statement::Instruction(ast::Instruction::Tex(
                tex,
                ast::Arg5Tex {
                    dst,
                    image,
                    layer,
                    coordinates,
                    lod,
                },
            )) => {
                if let Some(StateSpaceRemapping::ToBlock(id, ast::StateSpace::Global, offset)) =
                    var_tracker.get_remapped_state_space(fn_name, image)?
                {
                    let (image, _) = get_new_id_space(
                        StateSpaceRemapping::ToBlock(id, ast::StateSpace::Global, offset),
                        id_defs,
                        &mut result,
                        get_variable_pointer,
                    )?;
                    result.push(Statement::Instruction(ast::Instruction::Tex(
                        tex,
                        ast::Arg5Tex {
                            dst,
                            image,
                            layer,
                            coordinates,
                            lod,
                        },
                    )));
                } else {
                    return Err(TranslateError::unreachable());
                }
            }
            Statement::Instruction(inst) => {
                let new_statement = inst.visit(&mut |desc: ArgumentDescriptor<Id>,
                                                      typ: Option<(
                    &ast::Type,
                    ast::StateSpace,
                )>| {
                    let (expected_type_, expected_state_space) = match typ {
                        Some((t, s)) => (t, s),
                        None => return Ok(desc.op),
                    };
                    if desc.is_dst {
                        return Ok(desc.op);
                    }
                    match var_tracker.get_remapped_state_space(fn_name, desc.op)? {
                        None => Ok(desc.op),
                        Some(remapping) => {
                            let (override_id, override_space) = get_new_id_space(
                                remapping,
                                id_defs,
                                &mut result,
                                get_variable_pointer,
                            )?;
                            let (override_type, _, _, _) = id_defs.get_typed(override_id)?;
                            if expected_type_ != &override_type
                                || expected_state_space != override_space
                            {
                                return Err(TranslateError::unreachable());
                            }
                            Ok(override_id)
                        }
                    }
                })?;
                result.push(new_statement);
            }
            _ => result.push(statement),
        }
    }
    Ok(result)
}

pub(crate) fn replace_tex_builtins_hack<'a, 'input>(
    mut translation_module: TranslationModule<'input, ExpandedArgParams>,
) -> Result<TranslationModule<'input, ExpandedArgParams>, TranslateError> {
    for directive in translation_module.directives.iter_mut() {
        if let TranslationDirective::Method(TranslationMethod {
            source_name: Some(source_name),
            ..
        }) = directive
        {
            if source_name.starts_with("__zluda_ptx_impl__tex_") {
                *source_name = Cow::Owned(
                    source_name.replace("__zluda_ptx_impl__tex_", "__zluda_rt_ptx_impl__tex_"),
                )
            }
        }
    }
    Ok(translation_module)
}

fn get_new_id_space(
    remapping: StateSpaceRemapping,
    id_defs: &mut IdNameMapBuilder,
    result: &mut Vec<Statement<ast::Instruction<ExpandedArgParams>, ExpandedArgParams>>,
    (get_variable_pointer_global, get_variable_pointer_shared): (Id, Id),
) -> Result<(Id, ast::StateSpace), TranslateError> {
    let (override_id, override_space) = match remapping {
        StateSpaceRemapping::ToVariable(new_id) => {
            let (type_, new_state_space, _, _) = id_defs.get_typed(new_id)?;
            if new_state_space == ast::StateSpace::Reg {
                (
                    copy_value_get_local_pointer(id_defs, result, (new_id, type_)),
                    ast::StateSpace::Local,
                )
            } else {
                (new_id, new_state_space)
            }
        }
        StateSpaceRemapping::ToBlock(var_block, var_block_space, offset) => {
            let get_variable_pointer = if var_block_space == ast::StateSpace::Global {
                get_variable_pointer_global
            } else {
                get_variable_pointer_shared
            };
            (
                insert_call_to_get_variable_pointer(
                    id_defs,
                    var_block,
                    (get_variable_pointer, var_block_space),
                    result,
                    offset,
                ),
                var_block_space,
            )
        }
    };
    Ok((override_id, override_space))
}

fn copy_value_get_local_pointer(
    id_defs: &mut IdNameMapBuilder,
    result: &mut Vec<ExpandedStatement>,
    new_variable: (Id, ast::Type),
) -> Id {
    let (new_id, variable_type) = new_variable;
    let temp_var =
        id_defs.register_variable_def(None, variable_type.clone(), ast::StateSpace::Local, None);
    let temp_var_name = temp_var.name;
    result.push(Statement::Variable(temp_var));
    result.push(Statement::StoreVar(StoreVarDetails {
        arg: ast::Arg2St {
            src1: temp_var_name,
            src2: new_id,
        },
        type_: variable_type.clone(),
        member_index: None,
    }));
    temp_var_name
}

fn collect_variable_and_attribute_declarations<'a, 'input>(
    translation_module: &TranslationModule<'input, ExpandedArgParams>,
    raytracing_state: &RaytracingTranslationState<'a, 'input>,
) -> Result<VariablesLayout<'input>, TranslateError> {
    let mut result = RaytracingVariableTracker::new();
    let mut default_values = FxHashMap::<Id, _>::default();
    for directive in translation_module.directives.iter() {
        if let TranslationDirective::Variable(_, _, variable) = directive {
            if let Some(demangled) =
                demangle(&translation_module.id_defs.globals.reverse_variables[&variable.name])
            {
                if demangled.starts_with("rti_internal_semantic::") {
                    let global_name_str = &demangled[23..];
                    let global_name_id = translation_module
                        .id_defs
                        .globals
                        .variables
                        .get(global_name_str)
                        .ok_or_else(TranslateError::unexpected_pattern)?;
                    let (type_, state_space, align, _) =
                        translation_module.id_defs.get_typed(*global_name_id)?;
                    if state_space != ast::StateSpace::Global {
                        return Err(TranslateError::unexpected_pattern());
                    }
                    let default_value = default_values.get(global_name_id).copied();
                    let semantic = VariableSemantic::new(
                        global_name_str,
                        type_,
                        align,
                        &variable.initializer,
                        default_value,
                    )?;
                    result.record_variable(*global_name_id, semantic);
                } else {
                    default_values.insert(variable.name, &variable.initializer);
                }
            }
            // Texref variables are emitted as plain globals, without metadata
            else if variable.type_ == ast::Type::Texref {
                let text_name = translation_module
                    .id_defs
                    .globals
                    .reverse_variables
                    .get(&variable.name)
                    .ok_or_else(TranslateError::unexpected_pattern)?;
                result.record_variable(
                    variable.name,
                    VariableSemantic::Empty {
                        name: text_name.clone(),
                        type_: variable.type_.clone(),
                        align: None,
                        default_value: Vec::new(),
                    },
                );
            } else {
                default_values.insert(variable.name, &variable.initializer);
            }
        }
    }
    for (buffer_id, buffer_str_name) in raytracing_state.buffers.iter() {
        let buffer_type = ast::Type::Struct(
            BUFFER_OBJECT_FIELDS
                .iter()
                .copied()
                .map(ast::StructField::Scalar)
                .collect(),
        );
        result.record_variable(
            *buffer_id,
            VariableSemantic::Empty {
                name: buffer_str_name.clone(),
                type_: buffer_type,
                align: None,
                default_value: Vec::new(),
            },
        );
    }
    result.to_layout(raytracing_state)
}

struct RaytracingVariableTracker<'input> {
    // All declared globals (empty semantic variables, user attributes, builtin attributes)
    // We don't need extra information about builtin attributes, we already know their size
    // and they are not part of a memory block, but are converted from injected arguments
    global_to_semantic: FxHashMap<Id, VariableSemantic<'input>>,
}

impl<'input> RaytracingVariableTracker<'input> {
    fn new() -> Self {
        Self {
            global_to_semantic: FxHashMap::default(),
        }
    }

    fn record_variable(&mut self, id: Id, sema: VariableSemantic<'input>) {
        self.global_to_semantic.insert(id, sema);
    }

    fn to_layout(
        self,
        raytracing_state: &RaytracingTranslationState,
    ) -> Result<VariablesLayout<'input>, TranslateError> {
        fn effective_layout(
            align: &Option<u32>,
            type_: &ast::Type,
        ) -> Result<Layout, TranslateError> {
            Ok(align
                .map(|align| Layout::from_size_align(type_.layout().size(), align as usize))
                .transpose()
                .map_err(|_| TranslateError::unreachable())?
                .unwrap_or(type_.layout()))
        }
        let mut globals = Vec::new();
        let mut user_attributes = Vec::new();
        for (id, semantic) in self.global_to_semantic.iter() {
            if semantic.is_builtin() {
                continue;
            } else if let VariableSemantic::Empty {
                name,
                type_,
                align,
                default_value,
            } = semantic
            {
                let layout = effective_layout(align, type_)?;
                globals.push((*id, &*name, layout, default_value.clone()));
            } else if let VariableSemantic::Attribute { name, type_, align } = semantic {
                let layout = effective_layout(align, type_)?;
                user_attributes.push((*id, &*name, layout, Vec::new()));
            }
        }
        let variable_block_layout_zero = match raytracing_state.entry_point_kind {
            None => return Err(TranslateError::unreachable()),
            Some(RaytracingEntryPointKind::Intersection) => {
                let first_field = Layout::new::<*const ()>();
                first_field
                    .extend(Layout::new::<u32>())
                    .map_err(|_| TranslateError::unreachable())?
                    .0
            }
            Some(RaytracingEntryPointKind::BoundingBox) => Layout::new::<()>(),
            Some(RaytracingEntryPointKind::Unknown | RaytracingEntryPointKind::Callable) => {
                Layout::new::<*mut ()>()
            }
        };
        let mut offsets = FxHashMap::default();
        let globals = Self::compute_variables_block_impl(
            VariablesBlock {
                variables: FxHashMap::default(),
                layout: variable_block_layout_zero,
            },
            globals,
            &mut offsets,
        )?;
        let user_attributes = Self::compute_variables_block_impl(
            raytracing_state.old_attribute_variables.clone(),
            user_attributes,
            &mut offsets,
        )?;
        Ok(VariablesLayout {
            tracker: self,
            offsets,
            globals,
            user_attributes,
        })
    }

    fn compute_variables_block_impl<T: AsRef<str>>(
        mut var_block: VariablesBlock,
        variables: Vec<(Id, T, Layout, Vec<u8>)>,
        offsets: &mut FxHashMap<Id, u32>,
    ) -> Result<VariablesBlock, TranslateError> {
        let mut new_variables = variables
            .into_iter()
            .map(|(id, name, layout, default_value)| {
                let name =
                    CString::new(name.as_ref()).map_err(|_| TranslateError::unreachable())?;
                Ok(match var_block.variables.get(&name) {
                    Some(Variable { offset, .. }) => {
                        offsets.insert(id, *offset);
                        None
                    }
                    None => Some((id, name, layout, default_value)),
                })
            })
            .map(Result::transpose)
            .filter_map(convert::identity)
            .collect::<Result<Vec<_>, _>>()?;
        new_variables.sort_by_key(|(_, _, layout, _)| cmp::Reverse(layout.align()));
        for (id, name, var_layout, default_value) in new_variables.into_iter() {
            let (new_layout, offset) = var_block
                .layout
                .extend(var_layout)
                .map_err(|_| TranslateError::unreachable())?;
            offsets.insert(id, offset as u32);
            var_block.variables.insert(
                name,
                Variable {
                    size: var_layout.size() as u32,
                    offset: offset as u32,
                    default_value,
                },
            );
            var_block.layout = new_layout;
        }
        Ok(var_block)
    }
}

struct VariablesLayout<'input> {
    tracker: RaytracingVariableTracker<'input>,
    offsets: FxHashMap<Id, u32>,
    globals: VariablesBlock,
    // This block is the layout of *all* attributes,
    // both ones defined and ones defined in this module
    user_attributes: VariablesBlock,
}

struct FinalizedVariablesLayout<'input> {
    variables: VariablesLayout<'input>,
    injected_args: FxHashMap<Id, InjectedArguments>,
}

impl<'input> FinalizedVariablesLayout<'input> {
    fn remap_fn_arguments(
        &mut self,
        kind: RaytracingEntryPointKind,
        injected_args: &mut InjectedArguments,
        mut f: impl FnMut(ast::Type, ast::StateSpace) -> Id,
    ) {
        if kind == RaytracingEntryPointKind::BoundingBox {
            return;
        }
        self.remap_single_fn_argument(&mut f, injected_args, VariableSemantic::LaunchIndex);
        if kind == RaytracingEntryPointKind::Callable {
            return;
        }
        self.remap_single_fn_argument(&mut f, injected_args, VariableSemantic::LaunchDim);
        self.remap_single_fn_argument(&mut f, injected_args, VariableSemantic::CurrentRay);
        self.remap_single_fn_argument(&mut f, injected_args, VariableSemantic::CurrentTime);
        self.remap_single_fn_argument(&mut f, injected_args, VariableSemantic::Payload);
        self.remap_single_fn_argument(
            &mut f,
            injected_args,
            VariableSemantic::IntersectionDistance,
        );
    }

    fn remap_single_fn_argument(
        &mut self,
        f: &mut impl FnMut(ast::Type, ast::StateSpace) -> Id,
        injected_args: &mut InjectedArguments,
        sema: VariableSemantic<'input>,
    ) {
        let injected_arg = sema.to_injected_arg();
        let (type_, space) = injected_arg.get_type_space();
        let new_id = f(type_, space);
        injected_args.set(sema.to_injected_arg(), new_id);
    }

    fn get_remapped_state_space(
        &self,
        func: Id,
        id: Id,
    ) -> Result<Option<StateSpaceRemapping>, TranslateError> {
        let func_injected_args = self
            .injected_args
            .get(&func)
            .ok_or_else(TranslateError::unreachable)?;
        Ok(match self.variables.tracker.global_to_semantic.get(&id) {
            Some(semantic) => {
                if semantic.is_builtin() {
                    let arg_kind = semantic.to_injected_arg();
                    Some(StateSpaceRemapping::ToVariable(
                        func_injected_args
                            .get(arg_kind)
                            .ok_or_else(TranslateError::unreachable)?,
                    ))
                } else if let VariableSemantic::Empty { .. } = semantic {
                    let block = func_injected_args
                        .get(InjectedArgument::VariablesBlock)
                        .ok_or_else(TranslateError::unreachable)?;
                    let offset = self
                        .variables
                        .offsets
                        .get(&id)
                        .ok_or_else(TranslateError::unreachable)?;
                    Some(StateSpaceRemapping::ToBlock(
                        block,
                        ast::StateSpace::Global,
                        *offset,
                    ))
                } else if let VariableSemantic::Attribute { .. } = semantic {
                    let block = func_injected_args
                        .get(InjectedArgument::AttributesBlock)
                        .ok_or_else(TranslateError::unreachable)?;
                    let offset = self
                        .variables
                        .offsets
                        .get(&id)
                        .ok_or_else(TranslateError::unreachable)?;
                    Some(StateSpaceRemapping::ToBlock(
                        block,
                        ast::StateSpace::Global,
                        *offset,
                    ))
                } else {
                    return Err(TranslateError::unreachable());
                }
            }
            None => None,
        })
    }

    fn empty_semantic_variable_offset(&self, id: Id) -> Result<Option<u32>, TranslateError> {
        Ok(match self.variables.tracker.global_to_semantic.get(&id) {
            Some(VariableSemantic::Empty { .. }) => Some(
                self.variables
                    .offsets
                    .get(&id)
                    .copied()
                    .ok_or_else(TranslateError::unreachable)?,
            ),
            Some(_) => None,
            None => None,
        })
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum StateSpaceRemapping {
    ToVariable(Id),
    ToBlock(Id, ast::StateSpace, u32),
}

#[derive(Clone)]
enum VariableSemantic<'input> {
    Empty {
        name: Cow<'input, str>,
        type_: ast::Type,
        align: Option<u32>,
        default_value: Vec<u8>,
    },
    LaunchDim,
    LaunchIndex,
    Payload,
    CurrentRay,
    IntersectionDistance,
    CurrentTime,
    Attribute {
        name: String,
        type_: ast::Type,
        align: Option<u32>,
    },
}

impl<'a, 'input> VariableSemantic<'input> {
    fn new(
        global_name: &str,
        type_: ast::Type,
        align: Option<u32>,
        declaration_initializer: &Option<ast::Initializer<Id>>,
        definition_initializer: Option<&Option<ast::Initializer<Id>>>,
    ) -> Result<Self, TranslateError> {
        match declaration_initializer {
            None => {
                let default_value = match definition_initializer {
                    Some(Some(init)) => initializer_to_binary_vector(&type_, init)?,
                    _ => Vec::new(),
                };
                return Ok(VariableSemantic::Empty {
                    name: Cow::Owned(global_name.to_string()),
                    type_,
                    align,
                    default_value,
                });
            }
            Some(declaration_initializer) => {
                let declaration_initializer = initializer_to_binary_vector(
                    &ast::Type::Array(ast::ScalarType::U8, vec![0]),
                    declaration_initializer,
                )?;
                let text = unsafe { CStr::from_ptr(declaration_initializer.as_ptr() as _) }
                    .to_str()
                    .map_err(|_| TranslateError::unreachable())?;
                if text.starts_with("attribute ") {
                    return Ok(VariableSemantic::Attribute {
                        name: text[10..].to_string(),
                        type_,
                        align,
                    });
                }
                Ok(match text {
                    "rtLaunchDim" => VariableSemantic::LaunchDim,
                    "rtLaunchIndex" => VariableSemantic::LaunchIndex,
                    "rtPayload" => VariableSemantic::Payload,
                    "rtCurrentRay" => VariableSemantic::CurrentRay,
                    "rtIntersectionDistance" => VariableSemantic::IntersectionDistance,
                    "rtCurrentTime" => VariableSemantic::CurrentTime,
                    _ => return Err(TranslateError::todo()),
                })
            }
        }
    }

    fn is_builtin(&self) -> bool {
        match self {
            VariableSemantic::Empty { .. } | VariableSemantic::Attribute { .. } => false,
            VariableSemantic::LaunchDim
            | VariableSemantic::LaunchIndex
            | VariableSemantic::CurrentRay
            | VariableSemantic::CurrentTime
            | VariableSemantic::IntersectionDistance
            | VariableSemantic::Payload => true,
        }
    }

    fn to_injected_arg(&self) -> InjectedArgument {
        match self {
            VariableSemantic::Empty { .. } => InjectedArgument::VariablesBlock,
            VariableSemantic::LaunchDim => InjectedArgument::LaunchDim,
            VariableSemantic::LaunchIndex => InjectedArgument::LaunchIndex,
            VariableSemantic::Payload => InjectedArgument::Payload,
            VariableSemantic::CurrentRay => InjectedArgument::CurrentRay,
            VariableSemantic::CurrentTime => InjectedArgument::CurrentTime,
            VariableSemantic::IntersectionDistance => InjectedArgument::IntersectionDistance,
            VariableSemantic::Attribute { .. } => InjectedArgument::AttributesBlock,
        }
    }
}

fn initializer_to_binary_vector(
    type_: &ast::Type,
    initializer: &ast::Initializer<Id>,
) -> Result<Vec<u8>, TranslateError> {
    fn initializer_to_binary_vector_impl(
        type_: ConstType,
        initializer: &ast::Initializer<Id>,
        result: &mut Vec<u8>,
    ) -> Result<(), TranslateError> {
        match (type_, initializer) {
            (ConstType::Type(ast::Type::Array(scalar, dims)), ast::Initializer::Array(vec)) => {
                for initializer in vec {
                    initializer_to_binary_vector_impl(
                        ConstType::ArraySubtype(*scalar, &dims[1..]),
                        initializer,
                        result,
                    )?;
                }
            }
            (ConstType::ArraySubtype(scalar, dims), ast::Initializer::Array(vec)) => {
                for initializer in vec {
                    initializer_to_binary_vector_impl(
                        ConstType::ArraySubtype(scalar, &dims[1..]),
                        initializer,
                        result,
                    )?;
                }
            }
            (ConstType::ArraySubtype(ref scalar, []), ast::Initializer::Constant(constant))
            | (
                ConstType::Type(ast::Type::Scalar(ref scalar)),
                ast::Initializer::Constant(constant),
            ) => match scalar {
                ast::ScalarType::B8 | ast::ScalarType::U8 => {
                    let bytes = constant
                        .as_u8()
                        .ok_or_else(TranslateError::unexpected_pattern)?
                        .to_le_bytes();
                    result.extend_from_slice(&bytes);
                }
                ast::ScalarType::S8 => {
                    let bytes = constant
                        .as_i8()
                        .ok_or_else(TranslateError::unexpected_pattern)?
                        .to_le_bytes();
                    result.extend_from_slice(&bytes);
                }
                ast::ScalarType::B16 | ast::ScalarType::U16 => {
                    let bytes = constant
                        .as_u16()
                        .ok_or_else(TranslateError::unexpected_pattern)?
                        .to_le_bytes();
                    result.extend_from_slice(&bytes);
                }
                ast::ScalarType::S16 => {
                    let bytes = constant
                        .as_i16()
                        .ok_or_else(TranslateError::unexpected_pattern)?
                        .to_le_bytes();
                    result.extend_from_slice(&bytes);
                }
                ast::ScalarType::B32 | ast::ScalarType::U32 => {
                    let bytes = constant
                        .as_u32()
                        .ok_or_else(TranslateError::unexpected_pattern)?
                        .to_le_bytes();
                    result.extend_from_slice(&bytes);
                }
                ast::ScalarType::S32 => {
                    let bytes = constant
                        .as_i32()
                        .ok_or_else(TranslateError::unexpected_pattern)?
                        .to_le_bytes();
                    result.extend_from_slice(&bytes);
                }
                ast::ScalarType::B64 | ast::ScalarType::U64 => {
                    let bytes = constant
                        .as_u64()
                        .ok_or_else(TranslateError::unexpected_pattern)?
                        .to_le_bytes();
                    result.extend_from_slice(&bytes);
                }
                ast::ScalarType::S64 => {
                    let bytes = constant
                        .as_i64()
                        .ok_or_else(TranslateError::unexpected_pattern)?
                        .to_le_bytes();
                    result.extend_from_slice(&bytes);
                }
                ast::ScalarType::F16 => return Err(TranslateError::unexpected_pattern()),
                ast::ScalarType::F32 => {
                    let bytes = constant
                        .as_f32()
                        .ok_or_else(TranslateError::unexpected_pattern)?
                        .to_le_bytes();
                    result.extend_from_slice(&bytes);
                }
                ast::ScalarType::F64 => {
                    let bytes = constant
                        .as_f64()
                        .ok_or_else(TranslateError::unexpected_pattern)?
                        .to_le_bytes();
                    result.extend_from_slice(&bytes);
                }
                ast::ScalarType::F16x2 | ast::ScalarType::Pred => {
                    return Err(TranslateError::unexpected_pattern())
                }
            },
            _ => return Err(TranslateError::unexpected_pattern()),
        }
        Ok(())
    }
    let mut result = Vec::new();
    initializer_to_binary_vector_impl(ConstType::Type(type_), initializer, &mut result)?;
    Ok(result)
}

fn is_raytracing_entry_point<'a, 'input>(
    source_name: &'a Option<Cow<'input, str>>,
    entry_point: &str,
    suffixed_entry_point: &str,
) -> Result<Option<&'a Cow<'input, str>>, TranslateError> {
    if let Some(ref method_name) = source_name {
        if method_name != entry_point {
            if let Some(demangled) = demangle(&*method_name) {
                if !demangled.starts_with(suffixed_entry_point) {
                    return Ok(None);
                }
            } else {
                return Ok(None);
            }
        }
        Ok(Some(method_name))
    } else {
        Ok(None)
    }
}

fn demangle(input: &str) -> Option<String> {
    let symbol = BorrowedSymbol::new(input.as_bytes()).ok()?;
    symbol.demangle(&DemangleOptions::new()).ok()
}

pub(crate) fn bitcode() -> &'static [u8] {
    ZLUDA_RT_PTX_IMPL_AMD
}

pub struct Module<'input> {
    pub compilation_module: crate::translate::Module<'input>,
    pub kernel_source: &'static str,
    pub variables: VariablesBlock,
    pub attribute_variables: VariablesBlock,
    pub headers: Vec<Cow<'static, str>>,
    pub header_names: Vec<&'static CStr>,
    pub is_callable: bool,
    pub linker_module: Vec<u8>,
}

impl<'input> Module<'input> {
    pub const KERNEL_BOUNDING_BOX_NAME: &'static CStr = unsafe {
        CStr::from_bytes_with_nul_unchecked(b"__zluda_rt_ptx_impl__generate_bounding_box\0")
    };
    pub const FUNCTION_POINTER_NAME: &'static CStr =
        unsafe { CStr::from_bytes_with_nul_unchecked(b"__zluda_rt_ptx_impl__zluda_rt_func\0") };
    pub const ATTRIBUTE_FUNCTION_POINTER_NAME: &'static CStr = unsafe {
        CStr::from_bytes_with_nul_unchecked(b"__zluda_rt_ptx_impl__zluda_rt_attribute_func\0")
    };
    pub const KERNEL_NAME: &'static CStr =
        unsafe { CStr::from_bytes_with_nul_unchecked(b"__zluda_rt_ptx_impl__zluda_rt_kernel\0") };

    pub(crate) fn new(
        internal_kernel_name: String,
        compilation_module: crate::translate::Module<'input>,
        variables: VariablesBlock,
        kind: RaytracingEntryPointKind,
        attribute_variables: VariablesBlock,
        linker_module: Vec<u8>,
    ) -> Self {
        let template = match kind {
            RaytracingEntryPointKind::BoundingBox => KERNEL_SOURCE_BOUNDING_BOX,
            RaytracingEntryPointKind::Intersection => KERNEL_SOURCE_INTERSECT,
            RaytracingEntryPointKind::Unknown => KERNEL_SOURCE,
            RaytracingEntryPointKind::Callable => KERNEL_SOURCE_CALLABLE,
        };
        let unique_header = format!(
            "#define FUNCTION_NAME {}
            #define EXPORTED_FUNCTION {}
            #define EXPORTED_ATTRIBUTE_FUNCTION {}
            #define EXPORTED_KERNEL {}\0",
            internal_kernel_name,
            Self::FUNCTION_POINTER_NAME.to_string_lossy(),
            Self::ATTRIBUTE_FUNCTION_POINTER_NAME.to_string_lossy(),
            Self::KERNEL_NAME.to_string_lossy()
        );
        let headers = vec![Cow::Owned(unique_header), Cow::Borrowed(KERNEL_HEADER)];
        let header_names = unsafe {
            vec![
                CStr::from_bytes_with_nul_unchecked(b"raytracing_defines.hpp\0"),
                CStr::from_bytes_with_nul_unchecked(b"raytracing.hpp\0"),
            ]
        };
        Module {
            compilation_module,
            kernel_source: template,
            variables,
            attribute_variables,
            headers,
            header_names,
            is_callable: kind == RaytracingEntryPointKind::Callable,
            linker_module,
        }
    }
}

pub(crate) struct InjectedArguments(Vec<Option<Id>>);

impl InjectedArguments {
    fn new() -> Self {
        InjectedArguments(vec![None; InjectedArgument::_Count as usize])
    }

    fn set(&mut self, arg: InjectedArgument, id: Id) {
        self.0[arg as usize] = Some(id);
    }

    fn get(&self, arg: InjectedArgument) -> Option<Id> {
        self.0[arg as usize]
    }

    fn find_by_id(&self, needle: Id) -> Option<InjectedArgument> {
        self.0
            .iter()
            .copied()
            .position(|maybe_id| maybe_id == Some(needle))
            .map(|index| unsafe { mem::transmute(index as u8) })
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub(crate) enum InjectedArgument {
    GlobalState,
    PrimitiveIndex,
    LaunchIndex,
    LaunchDim,
    CurrentRay,
    CurrentTime,
    Payload,
    TriangleBarycentrics,
    TriangleNormals,
    VariablesBlock,
    AttributesBlock,
    TransformBlock,
    NewDistance,
    IntersectionResult,
    IntersectionDistance,
    MaterialResult,
    // TODO: replace with mem::variant_count when it stabilizes
    #[doc(hidden)]
    _Count,
}

impl InjectedArgument {
    fn get_type_space(self) -> (ast::Type, ast::StateSpace) {
        match self {
            InjectedArgument::GlobalState => (
                ast::Type::Scalar(ast::ScalarType::B8),
                ast::StateSpace::Shared,
            ),
            InjectedArgument::PrimitiveIndex => (
                ast::Type::Scalar(ast::ScalarType::B32),
                ast::StateSpace::Reg,
            ),
            InjectedArgument::LaunchIndex | InjectedArgument::LaunchDim => (
                ast::Type::Vector(ast::ScalarType::B32, 2),
                ast::StateSpace::Reg,
            ),
            InjectedArgument::CurrentRay => (
                ast::Type::Array(ast::ScalarType::B8, vec![36]),
                ast::StateSpace::Local,
            ),
            InjectedArgument::CurrentTime => (
                ast::Type::Scalar(ast::ScalarType::F32),
                ast::StateSpace::Reg,
            ),
            InjectedArgument::Payload => (
                ast::Type::Scalar(ast::ScalarType::B8),
                ast::StateSpace::Local,
            ),
            InjectedArgument::TriangleBarycentrics => (
                ast::Type::Vector(ast::ScalarType::F32, 2),
                ast::StateSpace::Reg,
            ),
            InjectedArgument::TriangleNormals => (
                ast::Type::Vector(ast::ScalarType::F32, 3),
                ast::StateSpace::Reg,
            ),
            InjectedArgument::VariablesBlock => (
                ast::Type::Scalar(ast::ScalarType::B8),
                ast::StateSpace::Global,
            ),
            InjectedArgument::AttributesBlock => (
                ast::Type::Scalar(ast::ScalarType::B8),
                ast::StateSpace::Global,
            ),
            InjectedArgument::TransformBlock => (
                ast::Type::Scalar(ast::ScalarType::B8),
                ast::StateSpace::Global,
            ),
            InjectedArgument::NewDistance => (
                ast::Type::Scalar(ast::ScalarType::F32),
                ast::StateSpace::Local,
            ),
            InjectedArgument::IntersectionResult => (
                ast::Type::Scalar(ast::ScalarType::B32),
                ast::StateSpace::Local,
            ),
            InjectedArgument::IntersectionDistance => (
                ast::Type::Scalar(ast::ScalarType::F32),
                ast::StateSpace::Reg,
            ),
            InjectedArgument::MaterialResult => (
                ast::Type::Scalar(ast::ScalarType::B32),
                ast::StateSpace::Local,
            ),
            InjectedArgument::_Count => unreachable!(),
        }
    }
}
