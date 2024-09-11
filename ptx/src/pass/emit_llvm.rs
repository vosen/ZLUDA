use std::convert::{TryFrom, TryInto};
use std::ptr;

use super::*;
use llvm_zluda::inkwell::builder::{Builder, BuilderError};
use llvm_zluda::inkwell::context::{AsContextRef, Context};
use llvm_zluda::inkwell::memory_buffer::MemoryBuffer;
use llvm_zluda::inkwell::types::{
    ArrayType, AsTypeRef, BasicMetadataTypeEnum, BasicType, BasicTypeEnum, FloatType, FunctionType,
    IntType, PointerType, VectorType, VoidType,
};
use llvm_zluda::inkwell::values::{
    AnyValue, AnyValueEnum, ArrayValue, BasicValueEnum, FloatMathValue, FloatValue, FunctionValue,
    InstructionValue, IntMathValue, IntValue, PhiValue, PointerValue, StructValue, VectorValue,
};
use llvm_zluda::inkwell::{self, module, AddressSpace};
use llvm_zluda::llvm::core::{
    LLVMArrayType2, LLVMBFloatType, LLVMBFloatTypeInContext, LLVMVectorType,
};
use llvm_zluda::llvm::prelude::*;
use llvm_zluda::llvm::{LLVMCallConv, LLVMZludaBuildAlloca};

const LLVM_UNNAMED: &str = "\0";
// https://llvm.org/docs/AMDGPUUsage.html#address-spaces
const GENERIC_ADDRESS_SPACE: u16 = 0;
const GLOBAL_ADDRESS_SPACE: u16 = 1;
const SHARED_ADDRESS_SPACE: u16 = 3;
const CONSTANT_ADDRESS_SPACE: u16 = 4;
const PRIVATE_ADDRESS_SPACE: u16 = 5;

pub(super) fn run<'input>(
    id_defs: &GlobalStringIdResolver<'input>,
    call_map: MethodsCallMap<'input>,
    directives: Vec<Directive<'input>>,
) -> Result<MemoryBuffer, TranslateError> {
    let context = inkwell::context::Context::create();
    let module = context.create_module(LLVM_UNNAMED);
    let builder = context.create_builder();
    let mut emit_ctx = ModuleEmitContext::new(&context, module, builder, id_defs);
    for directive in directives {
        match directive {
            Directive::Variable(..) => todo!(),
            Directive::Method(method) => emit_ctx.emit_method(method)?,
        }
    }
    if let Err(err) = emit_ctx.module.verify() {
        emit_ctx.module.print_to_stderr();
        panic!("{}", err);
    }
    Ok(emit_ctx.module.write_bitcode_to_memory())
}

struct ModuleEmitContext<'ctx, 'input> {
    context: &'ctx Context,
    module: module::Module<'ctx>,
    builder: Builder<'ctx>,
    id_defs: &'ctx GlobalStringIdResolver<'input>,
    resolver: ResolveIdent<'ctx>,
}

impl<'ctx, 'input> ModuleEmitContext<'ctx, 'input> {
    fn new(
        context: &'ctx Context,
        module: module::Module<'ctx>,
        builder: Builder<'ctx>,
        id_defs: &'ctx GlobalStringIdResolver<'input>,
    ) -> Self {
        ModuleEmitContext {
            context: &context,
            module,
            builder,
            id_defs,
            resolver: ResolveIdent::new(&id_defs),
        }
    }

    fn kernel_call_convention() -> u32 {
        LLVMCallConv::LLVMAMDGPUKERNELCallConv as u32
    }

    fn func_call_convention() -> u32 {
        LLVMCallConv::LLVMCCallConv as u32
    }

    fn emit_method(&mut self, method: Function<'input>) -> Result<(), TranslateError> {
        let func_decl = method.func_decl.borrow();
        let fn_ = self.module.add_function(
            method
                .import_as
                .as_deref()
                .unwrap_or_else(|| match func_decl.name {
                    ast::MethodName::Kernel(name) => name,
                    ast::MethodName::Func(id) => self.id_defs.reverse_variables[&id],
                }),
            self.function_type(
                func_decl.return_arguments.iter().map(|v| &v.v_type),
                func_decl.input_arguments.iter().map(|v| &v.v_type),
            ),
            None,
        );
        for (i, param) in func_decl.input_arguments.iter().enumerate() {
            let value = fn_
                .get_nth_param(i as u32)
                .ok_or_else(|| error_unreachable())?;
            value.set_name(self.resolver.get_or_add(param.name));
            self.resolver.register(param.name, value);
        }
        fn_.set_call_conventions(if func_decl.name.is_kernel() {
            Self::kernel_call_convention()
        } else {
            Self::func_call_convention()
        });
        if let Some(statements) = method.body {
            let variables_bb = self.context.append_basic_block(fn_, LLVM_UNNAMED);
            let variables_builder = self.context.create_builder();
            variables_builder.position_at_end(variables_bb);
            let real_bb = self.context.append_basic_block(fn_, LLVM_UNNAMED);
            self.builder.position_at_end(real_bb);
            let mut method_emitter = MethodEmitContext::new(self, fn_, variables_builder);
            for statement in statements {
                method_emitter.emit_statement(statement)?;
            }
            method_emitter.variables_builder.build_unconditional_branch(real_bb);
        }
        Ok(())
    }

    fn function_type<'a>(
        &self,
        return_args: impl ExactSizeIterator<Item = &'a ast::Type>,
        input_args: impl ExactSizeIterator<Item = &'a ast::Type>,
    ) -> FunctionType<'ctx> {
        if return_args.len() == 0 {
            let input_args = input_args
                .map(|type_| match type_ {
                    ast::Type::Scalar(scalar) => match scalar {
                        ast::ScalarType::Pred => {
                            BasicMetadataTypeEnum::from(self.context.bool_type())
                        }
                        ast::ScalarType::S8 | ast::ScalarType::B8 | ast::ScalarType::U8 => {
                            BasicMetadataTypeEnum::from(self.context.i8_type())
                        }
                        ast::ScalarType::B16 | ast::ScalarType::U16 | ast::ScalarType::S16 => {
                            BasicMetadataTypeEnum::from(self.context.i16_type())
                        }
                        ast::ScalarType::S32 | ast::ScalarType::B32 | ast::ScalarType::U32 => {
                            BasicMetadataTypeEnum::from(self.context.i32_type())
                        }
                        ast::ScalarType::U64 | ast::ScalarType::S64 | ast::ScalarType::B64 => {
                            BasicMetadataTypeEnum::from(self.context.i64_type())
                        }
                        ast::ScalarType::B128 => {
                            BasicMetadataTypeEnum::from(self.context.i128_type())
                        }
                        ast::ScalarType::F16 => {
                            BasicMetadataTypeEnum::from(self.context.f16_type())
                        }
                        ast::ScalarType::F32 => {
                            BasicMetadataTypeEnum::from(self.context.f32_type())
                        }
                        ast::ScalarType::F64 => {
                            BasicMetadataTypeEnum::from(self.context.f64_type())
                        }
                        ast::ScalarType::BF16 => {
                            BasicMetadataTypeEnum::from(unsafe { FloatType::new(LLVMBFloatType()) })
                        }
                        ast::ScalarType::U16x2 => todo!(),
                        ast::ScalarType::S16x2 => todo!(),
                        ast::ScalarType::F16x2 => todo!(),
                        ast::ScalarType::BF16x2 => todo!(),
                    },
                    ast::Type::Vector(_, _) => todo!(),
                    ast::Type::Array(_, _, _) => todo!(),
                    ast::Type::Pointer(_, _) => todo!(),
                })
                .collect::<Vec<_>>();
            return self.context.void_type().fn_type(&*input_args, false);
        }
        todo!()
    }

    fn get_type(&self, type_: &ast::Type) -> FunctionType<'ctx> {
        match type_ {
            ast::Type::Scalar(_) => todo!(),
            ast::Type::Vector(_, _) => todo!(),
            ast::Type::Array(_, _, _) => todo!(),
            ast::Type::Pointer(_, _) => todo!(),
        }
    }
}

struct MethodEmitContext<'a, 'ctx, 'input> {
    context: &'ctx Context,
    module: &'a module::Module<'ctx>,
    method: FunctionValue<'ctx>,
    builder: &'a Builder<'ctx>,
    id_defs: &'a GlobalStringIdResolver<'input>,
    variables_builder: Builder<'ctx>,
    resolver: &'a mut ResolveIdent<'ctx>,
}

impl<'a, 'ctx, 'input> MethodEmitContext<'a, 'ctx, 'input> {
    fn new(
        parent: &'a mut ModuleEmitContext<'ctx, 'input>,
        method: FunctionValue<'ctx>,
        variables_builder: Builder<'ctx>,
    ) -> MethodEmitContext<'a, 'ctx, 'input> {
        MethodEmitContext {
            context: &parent.context,
            module: &parent.module,
            builder: &parent.builder,
            id_defs: parent.id_defs,
            variables_builder,
            resolver: &mut parent.resolver,
            method,
        }
    }

    fn emit_statement(
        &mut self,
        statement: Statement<ast::Instruction<SpirvWord>, SpirvWord>,
    ) -> Result<(), TranslateError> {
        Ok(match statement {
            Statement::Variable(var) => self.emit_variable(var)?,
            Statement::Label(label) => self.emit_label(label),
            Statement::Instruction(inst) => self.emit_instruction(inst)?,
            Statement::Conditional(_) => todo!(),
            Statement::LoadVar(var) => self.emit_load_variable(var)?,
            Statement::StoreVar(store) => self.emit_store_var(store)?,
            Statement::Conversion(conversion) => self.emit_conversion(conversion)?,
            Statement::Constant(constant) => self.emit_constant(constant)?,
            Statement::RetValue(_, _) => todo!(),
            Statement::PtrAccess(_) => todo!(),
            Statement::RepackVector(_) => todo!(),
            Statement::FunctionPointer(_) => todo!(),
        })
    }

    fn emit_variable(&mut self, var: ast::Variable<SpirvWord>) -> Result<(), TranslateError> {
        let alloca = unsafe {
            PointerValue::new(LLVMZludaBuildAlloca(
                self.variables_builder.as_mut_ptr(),
                get_type::<BasicTypeEnum>(&self.context, &var.v_type)?.as_type_ref(),
                get_state_space(var.state_space)? as u32,
                self.resolver.get_or_add_raw(var.name),
            ))
        };
        self.resolver.register(var.name, alloca);
        if let Some(align) = var.align {
            let alloca = alloca.as_instruction().ok_or_else(|| error_unreachable())?;
            alloca
                .set_alignment(align)
                .map_err(|_| error_unreachable())?;
        }
        if !var.array_init.is_empty() {
            todo!()
        }
        Ok(())
    }

    fn emit_label(&mut self, label: SpirvWord) {
        let block = self
            .context
            .append_basic_block(self.method, self.resolver.get_or_add(label));
        if self
            .builder
            .get_insert_block()
            .unwrap()
            .get_terminator()
            .is_none()
        {
            self.builder.build_unconditional_branch(block);
        }
        self.builder.position_at_end(block);
    }

    fn emit_store_var(&mut self, store: StoreVarDetails) -> Result<(), TranslateError> {
        let src1 = self.resolver.value(store.arg.src1)?;
        let src2 = self.resolver.value(store.arg.src2)?;
        self.builder
            .build_store(src1.as_pointer()?, src2.as_basic()?)
            .map_err(|_| error_unreachable())?;
        Ok(())
    }

    fn emit_instruction(
        &mut self,
        inst: ast::Instruction<SpirvWord>,
    ) -> Result<(), TranslateError> {
        match inst {
            ast::Instruction::Mov { data, arguments } => todo!(),
            ast::Instruction::Ld { data, arguments } => self.emit_ld(data, arguments),
            ast::Instruction::Add { data, arguments } => self.emit_add(data, arguments),
            ast::Instruction::St { data, arguments } => self.emit_st(data, arguments),
            ast::Instruction::Mul { data, arguments } => todo!(),
            ast::Instruction::Setp { data, arguments } => todo!(),
            ast::Instruction::SetpBool { data, arguments } => todo!(),
            ast::Instruction::Not { data, arguments } => todo!(),
            ast::Instruction::Or { data, arguments } => todo!(),
            ast::Instruction::And { data, arguments } => todo!(),
            ast::Instruction::Bra { arguments } => todo!(),
            ast::Instruction::Call { data, arguments } => todo!(),
            ast::Instruction::Cvt { data, arguments } => todo!(),
            ast::Instruction::Shr { data, arguments } => todo!(),
            ast::Instruction::Shl { data, arguments } => todo!(),
            ast::Instruction::Ret { data } => self.emit_ret(data),
            ast::Instruction::Cvta { data, arguments } => todo!(),
            ast::Instruction::Abs { data, arguments } => todo!(),
            ast::Instruction::Mad { data, arguments } => todo!(),
            ast::Instruction::Fma { data, arguments } => todo!(),
            ast::Instruction::Sub { data, arguments } => todo!(),
            ast::Instruction::Min { data, arguments } => todo!(),
            ast::Instruction::Max { data, arguments } => todo!(),
            ast::Instruction::Rcp { data, arguments } => todo!(),
            ast::Instruction::Sqrt { data, arguments } => todo!(),
            ast::Instruction::Rsqrt { data, arguments } => todo!(),
            ast::Instruction::Selp { data, arguments } => todo!(),
            ast::Instruction::Bar { data, arguments } => todo!(),
            ast::Instruction::Atom { data, arguments } => todo!(),
            ast::Instruction::AtomCas { data, arguments } => todo!(),
            ast::Instruction::Div { data, arguments } => todo!(),
            ast::Instruction::Neg { data, arguments } => todo!(),
            ast::Instruction::Sin { data, arguments } => todo!(),
            ast::Instruction::Cos { data, arguments } => todo!(),
            ast::Instruction::Lg2 { data, arguments } => todo!(),
            ast::Instruction::Ex2 { data, arguments } => todo!(),
            ast::Instruction::Clz { data, arguments } => todo!(),
            ast::Instruction::Brev { data, arguments } => todo!(),
            ast::Instruction::Popc { data, arguments } => todo!(),
            ast::Instruction::Xor { data, arguments } => todo!(),
            ast::Instruction::Rem { data, arguments } => todo!(),
            ast::Instruction::Bfe { data, arguments } => todo!(),
            ast::Instruction::Bfi { data, arguments } => todo!(),
            ast::Instruction::PrmtSlow { arguments } => todo!(),
            ast::Instruction::Prmt { data, arguments } => todo!(),
            ast::Instruction::Activemask { arguments } => todo!(),
            ast::Instruction::Membar { data } => todo!(),
            ast::Instruction::Trap {} => todo!(),
        }
    }

    fn emit_ld(
        &mut self,
        data: ast::LdDetails,
        arguments: ast::LdArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        if data.non_coherent {
            todo!()
        }
        if data.qualifier != ast::LdStQualifier::Weak {
            todo!()
        }
        let builder = self.builder;
        let type_ = get_type::<BasicTypeEnum>(&self.context, &data.typ)?;
        let ptr = self.resolver.value(arguments.src)?.as_pointer()?;
        self.resolver
            .with_result(arguments.dst, |dst| builder.build_load(type_, ptr, dst))
    }

    fn emit_load_variable(&mut self, var: LoadVarDetails) -> Result<(), TranslateError> {
        if var.member_index.is_some() {
            todo!()
        }
        let builder = self.builder;
        let type_ = get_type::<BasicTypeEnum>(&self.context, &var.typ)?;
        let ptr = self.resolver.value(var.arg.src)?.as_pointer()?;
        self.resolver
            .with_result(var.arg.dst, |dst| builder.build_load(type_, ptr, dst))
    }

    fn emit_conversion(&mut self, conversion: ImplicitConversion) -> Result<(), TranslateError> {
        let builder = self.builder;
        match conversion.kind {
            ConversionKind::Default => todo!(),
            ConversionKind::SignExtend => todo!(),
            ConversionKind::BitToPtr => {
                let src = self.resolver.value(conversion.src)?.as_int()?;
                let type_ = get_pointer_type(self.context, conversion.to_space)?;
                self.resolver.with_result(conversion.dst, |dst| {
                    builder.build_int_to_ptr(src, type_, dst)
                })
            }
            ConversionKind::PtrToPtr => todo!(),
            ConversionKind::AddressOf => todo!(),
        }
    }

    fn emit_constant(&mut self, constant: ConstantDefinition) -> Result<(), TranslateError> {
        let type_ = get_scalar_type::<BasicTypeEnum>(&self.context, constant.typ);
        let value: AnyValueEnum = match (type_, constant.value) {
            (BasicTypeEnum::IntType(type_), ast::ImmediateValue::U64(x)) => {
                type_.const_int(x, false).into()
            }
            (BasicTypeEnum::IntType(type_), ast::ImmediateValue::S64(x)) => {
                type_.const_int(x as u64, false).into()
            }
            (BasicTypeEnum::FloatType(type_), ast::ImmediateValue::F32(x)) => {
                type_.const_float(x as f64).into()
            }
            (BasicTypeEnum::FloatType(type_), ast::ImmediateValue::F64(x)) => {
                type_.const_float(x).into()
            }
            _ => return Err(error_unreachable()),
        };
        self.resolver.register(constant.dst, value);
        Ok(())
    }

    fn emit_add(
        &mut self,
        data: ast::ArithDetails,
        arguments: ast::AddArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let builder = self.builder;
        let src1 = self.resolver.value(arguments.src1)?.as_int()?;
        let src2 = self.resolver.value(arguments.src2)?.as_int()?;
        let fn_ = match data {
            ast::ArithDetails::Integer(integer) => Builder::build_int_add,
            ast::ArithDetails::Float(float) => todo!(),
        };
        self.resolver
            .with_result(arguments.dst, |dst| fn_(builder, src1, src2, dst))
    }

    fn emit_st(
        &self,
        data: ptx_parser::StData,
        arguments: ptx_parser::StArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let builder = self.builder;
        let src1 = self.resolver.value(arguments.src1)?.as_pointer()?;
        let src2 = self.resolver.value(arguments.src2)?.as_basic()?;
        if data.qualifier != ast::LdStQualifier::Weak {
            todo!()
        }
        self.builder
            .build_store(src1, src2)
            .map_err(|_| error_unreachable())?;
        Ok(())
    }

    fn emit_ret(&self, _data: ptx_parser::RetData) -> Result<(), TranslateError> {
        self.builder
            .build_return(None)
            .map_err(|_| error_unreachable())?;
        Ok(())
    }
}

fn get_pointer_type<'ctx>(
    context: &'ctx Context,
    to_space: ast::StateSpace,
) -> Result<PointerType<'ctx>, TranslateError> {
    Ok(context.ptr_type(AddressSpace::from(get_state_space(to_space)?)))
}

fn get_type<
    'ctx,
    T: From<IntType<'ctx>>
        + From<FloatType<'ctx>>
        + From<VectorType<'ctx>>
        + From<PointerType<'ctx>>
        + From<ArrayType<'ctx>>,
>(
    context: &'ctx Context,
    type_: &ast::Type,
) -> Result<T, TranslateError> {
    Ok(match type_ {
        ast::Type::Scalar(scalar) => get_scalar_type(context, *scalar),
        ast::Type::Vector(size, scalar) => {
            let base_type = get_scalar_type::<BasicTypeEnum>(context, *scalar);
            let base_type = match base_type {
                BasicTypeEnum::FloatType(t) => t.as_type_ref(),
                BasicTypeEnum::IntType(t) => t.as_type_ref(),
                _ => return Err(error_unreachable()),
            };
            T::from(unsafe { VectorType::new(LLVMVectorType(base_type, *size as u32)) })
        }
        ast::Type::Array(vec, scalar, dimensions) => {
            let mut underlying_type = get_scalar_type::<BasicTypeEnum>(context, *scalar);
            if let Some(size) = vec {
                underlying_type = BasicTypeEnum::VectorType(unsafe {
                    VectorType::new(LLVMVectorType(
                        match underlying_type {
                            BasicTypeEnum::FloatType(t) => t.as_type_ref(),
                            BasicTypeEnum::IntType(t) => t.as_type_ref(),
                            _ => return Err(error_unreachable()),
                        },
                        size.get() as u32,
                    ))
                });
            }
            if dimensions.is_empty() {
                return Ok(T::from(underlying_type.array_type(0)));
            }
            let llvm_type = dimensions
                .iter()
                .rfold(underlying_type.as_type_ref(), |result, dimension| unsafe {
                    LLVMArrayType2(result, *dimension as u64)
                });
            T::from(unsafe { ArrayType::new(llvm_type) })
        }
        ast::Type::Pointer(_, space) => {
            T::from(context.ptr_type(AddressSpace::from(get_state_space(*space)?)))
        }
    })
}

fn get_scalar_type<
    'ctx,
    T: From<IntType<'ctx>> + From<FloatType<'ctx>> + From<VectorType<'ctx>>,
>(
    context: &'ctx Context,
    type_: ast::ScalarType,
) -> T {
    match type_ {
        ast::ScalarType::Pred => T::from(context.bool_type()),
        ast::ScalarType::S8 | ast::ScalarType::B8 | ast::ScalarType::U8 => {
            T::from(context.i8_type())
        }
        ast::ScalarType::B16 | ast::ScalarType::U16 | ast::ScalarType::S16 => {
            T::from(context.i16_type())
        }
        ast::ScalarType::S32 | ast::ScalarType::B32 | ast::ScalarType::U32 => {
            T::from(context.i32_type())
        }
        ast::ScalarType::U64 | ast::ScalarType::S64 | ast::ScalarType::B64 => {
            T::from(context.i64_type())
        }
        ast::ScalarType::B128 => T::from(context.i128_type()),
        ast::ScalarType::F16 => T::from(context.f16_type()),
        ast::ScalarType::F32 => T::from(context.f32_type()),
        ast::ScalarType::F64 => T::from(context.f64_type()),
        ast::ScalarType::BF16 => {
            T::from(unsafe { FloatType::new(LLVMBFloatTypeInContext(context.as_ctx_ref())) })
        }
        ast::ScalarType::U16x2 | ast::ScalarType::S16x2 => {
            T::from(unsafe { VectorType::new(LLVMVectorType(context.i16_type().as_type_ref(), 2)) })
        }
        ast::ScalarType::F16x2 => {
            T::from(unsafe { VectorType::new(LLVMVectorType(context.f16_type().as_type_ref(), 2)) })
        }
        ast::ScalarType::BF16x2 => T::from(unsafe {
            VectorType::new(LLVMVectorType(
                LLVMBFloatTypeInContext(context.as_ctx_ref()),
                2,
            ))
        }),
    }
}

fn get_state_space(space: ast::StateSpace) -> Result<u16, TranslateError> {
    match space {
        ast::StateSpace::Reg => Ok(PRIVATE_ADDRESS_SPACE),
        ast::StateSpace::Generic => Ok(GENERIC_ADDRESS_SPACE),
        ast::StateSpace::Sreg => Ok(PRIVATE_ADDRESS_SPACE),
        ast::StateSpace::Param => Err(TranslateError::Todo),
        ast::StateSpace::ParamEntry => Err(TranslateError::Todo),
        ast::StateSpace::ParamFunc => Err(TranslateError::Todo),
        ast::StateSpace::Local => Ok(PRIVATE_ADDRESS_SPACE),
        ast::StateSpace::Global => Ok(GLOBAL_ADDRESS_SPACE),
        ast::StateSpace::Const => Ok(CONSTANT_ADDRESS_SPACE),
        ast::StateSpace::Shared => Ok(SHARED_ADDRESS_SPACE),
        ast::StateSpace::SharedCta => Err(TranslateError::Todo),
        ast::StateSpace::SharedCluster => Err(TranslateError::Todo),
    }
}

struct ResolveIdent<'ctx> {
    words: HashMap<SpirvWord, String>,
    values: HashMap<SpirvWord, AnyValueEnum<'ctx>>,
}

impl<'ctx> ResolveIdent<'ctx> {
    fn new<'input>(_id_defs: &GlobalStringIdResolver<'input>) -> Self {
        ResolveIdent {
            words: HashMap::new(),
            values: HashMap::new(),
        }
    }

    fn get_or_ad_impl<'a, T>(&'a mut self, word: SpirvWord, fn_: impl FnOnce(&'a str) -> T) -> T {
        match self.words.entry(word) {
            hash_map::Entry::Occupied(entry) => fn_(entry.into_mut()),
            hash_map::Entry::Vacant(entry) => {
                let mut text = word.0.to_string();
                text.push('\0');
                fn_(entry.insert(text))
            }
        }
    }

    fn get_or_add(&mut self, word: SpirvWord) -> &str {
        self.get_or_ad_impl(word, |x| x)
    }

    fn get_or_add_raw(&mut self, word: SpirvWord) -> *const i8 {
        self.get_or_add(word).as_ptr().cast()
    }

    fn register(&mut self, word: SpirvWord, t: impl AnyValue<'ctx>) {
        self.values.insert(word, t.as_any_value_enum());
    }

    fn value(&self, word: SpirvWord) -> Result<AnyValueEnum<'ctx>, TranslateError> {
        self.values
            .get(&word)
            .copied()
            .ok_or_else(|| error_unreachable())
    }

    fn with_result<T: AnyValue<'ctx>>(
        &mut self,
        word: SpirvWord,
        fn_: impl FnOnce(&str) -> Result<T, BuilderError>,
    ) -> Result<(), TranslateError> {
        let t = self
            .get_or_ad_impl(word, fn_)
            .map_err(|_| error_unreachable())?;
        self.register(word, t);
        Ok(())
    }

    fn build_int_math(
        &mut self,
        builder: &Builder<'ctx>,
        dst: SpirvWord,
        src1: SpirvWord,
        src2: SpirvWord,
        fn_: impl IntMathOp<'ctx>,
    ) -> Result<(), TranslateError> {
        let src1 = self.value(src1)?;
        let src2 = self.value(src2)?;
        self.with_result(dst, |dst| {
            Ok(match (src1, src2) {
                (AnyValueEnum::IntValue(src1), AnyValueEnum::IntValue(src2)) => {
                    AnyValueEnum::from(fn_.call(builder, src1, src2, dst)?)
                }
                (AnyValueEnum::PointerValue(src1), AnyValueEnum::PointerValue(src2)) => {
                    AnyValueEnum::from(fn_.call(builder, src1, src2, dst)?)
                }
                (AnyValueEnum::VectorValue(src1), AnyValueEnum::VectorValue(src2)) => {
                    AnyValueEnum::from(fn_.call(builder, src1, src2, dst)?)
                }
                _ => return todo!(),
            })
        })
    }
}

trait IntMathOp<'ctx> {
    fn call<T: IntMathValue<'ctx>>(
        self,
        builder: &Builder<'ctx>,
        src1: T,
        src2: T,
        dst: &str,
    ) -> Result<T, BuilderError>;
}

trait AnyValueEnumExt<'ctx> {
    fn as_array(self) -> Result<ArrayValue<'ctx>, TranslateError>;
    fn as_int(self) -> Result<IntValue<'ctx>, TranslateError>;
    fn as_float(self) -> Result<FloatValue<'ctx>, TranslateError>;
    fn as_phi(self) -> Result<PhiValue<'ctx>, TranslateError>;
    fn as_function(self) -> Result<FunctionValue<'ctx>, TranslateError>;
    fn as_pointer(self) -> Result<PointerValue<'ctx>, TranslateError>;
    fn as_struct(self) -> Result<StructValue<'ctx>, TranslateError>;
    fn as_vector(self) -> Result<VectorValue<'ctx>, TranslateError>;
    fn as_instruction(self) -> Result<InstructionValue<'ctx>, TranslateError>;
    fn as_basic(self) -> Result<BasicValueEnum<'ctx>, TranslateError>;
}

impl<'ctx> AnyValueEnumExt<'ctx> for AnyValueEnum<'ctx> {
    fn as_array(self) -> Result<ArrayValue<'ctx>, TranslateError> {
        if let AnyValueEnum::ArrayValue(x) = self {
            Ok(x)
        } else {
            Err(error_unreachable())
        }
    }

    fn as_int(self) -> Result<IntValue<'ctx>, TranslateError> {
        if let AnyValueEnum::IntValue(x) = self {
            Ok(x)
        } else {
            Err(error_unreachable())
        }
    }

    fn as_float(self) -> Result<FloatValue<'ctx>, TranslateError> {
        if let AnyValueEnum::FloatValue(x) = self {
            Ok(x)
        } else {
            Err(error_unreachable())
        }
    }

    fn as_phi(self) -> Result<PhiValue<'ctx>, TranslateError> {
        if let AnyValueEnum::PhiValue(x) = self {
            Ok(x)
        } else {
            Err(error_unreachable())
        }
    }

    fn as_function(self) -> Result<FunctionValue<'ctx>, TranslateError> {
        if let AnyValueEnum::FunctionValue(x) = self {
            Ok(x)
        } else {
            Err(error_unreachable())
        }
    }

    fn as_pointer(self) -> Result<PointerValue<'ctx>, TranslateError> {
        if let AnyValueEnum::PointerValue(x) = self {
            Ok(x)
        } else {
            Err(error_unreachable())
        }
    }

    fn as_struct(self) -> Result<StructValue<'ctx>, TranslateError> {
        if let AnyValueEnum::StructValue(x) = self {
            Ok(x)
        } else {
            Err(error_unreachable())
        }
    }

    fn as_vector(self) -> Result<VectorValue<'ctx>, TranslateError> {
        if let AnyValueEnum::VectorValue(x) = self {
            Ok(x)
        } else {
            Err(error_unreachable())
        }
    }

    fn as_instruction(self) -> Result<InstructionValue<'ctx>, TranslateError> {
        if let AnyValueEnum::InstructionValue(x) = self {
            Ok(x)
        } else {
            Err(error_unreachable())
        }
    }

    fn as_basic(self) -> Result<BasicValueEnum<'ctx>, TranslateError> {
        BasicValueEnum::try_from(self).map_err(|_| error_unreachable())
    }
}
