use ptx_parser as ast;
use std::{
    borrow::Cow,
    cell::RefCell,
    collections::{hash_map, HashMap},
    rc::Rc,
};

mod normalize;

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
enum PtxSpecialRegister {
    Tid,
    Ntid,
    Ctaid,
    Nctaid,
    Clock,
    LanemaskLt,
}

impl PtxSpecialRegister {
    fn try_parse(s: &str) -> Option<Self> {
        match s {
            "%tid" => Some(Self::Tid),
            "%ntid" => Some(Self::Ntid),
            "%ctaid" => Some(Self::Ctaid),
            "%nctaid" => Some(Self::Nctaid),
            "%clock" => Some(Self::Clock),
            "%lanemask_lt" => Some(Self::LanemaskLt),
            _ => None,
        }
    }

    fn get_type(self) -> ast::Type {
        match self {
            PtxSpecialRegister::Tid
            | PtxSpecialRegister::Ntid
            | PtxSpecialRegister::Ctaid
            | PtxSpecialRegister::Nctaid => ast::Type::Vector(self.get_function_return_type(), 4),
            _ => ast::Type::Scalar(self.get_function_return_type()),
        }
    }

    fn get_function_return_type(self) -> ast::ScalarType {
        match self {
            PtxSpecialRegister::Tid => ast::ScalarType::U32,
            PtxSpecialRegister::Ntid => ast::ScalarType::U32,
            PtxSpecialRegister::Ctaid => ast::ScalarType::U32,
            PtxSpecialRegister::Nctaid => ast::ScalarType::U32,
            PtxSpecialRegister::Clock => ast::ScalarType::U32,
            PtxSpecialRegister::LanemaskLt => ast::ScalarType::U32,
        }
    }

    fn get_function_input_type(self) -> Option<ast::ScalarType> {
        match self {
            PtxSpecialRegister::Tid
            | PtxSpecialRegister::Ntid
            | PtxSpecialRegister::Ctaid
            | PtxSpecialRegister::Nctaid => Some(ast::ScalarType::U8),
            PtxSpecialRegister::Clock | PtxSpecialRegister::LanemaskLt => None,
        }
    }

    fn get_unprefixed_function_name(self) -> &'static str {
        match self {
            PtxSpecialRegister::Tid => "sreg_tid",
            PtxSpecialRegister::Ntid => "sreg_ntid",
            PtxSpecialRegister::Ctaid => "sreg_ctaid",
            PtxSpecialRegister::Nctaid => "sreg_nctaid",
            PtxSpecialRegister::Clock => "sreg_clock",
            PtxSpecialRegister::LanemaskLt => "sreg_lanemask_lt",
        }
    }
}

struct SpecialRegistersMap {
    reg_to_id: HashMap<PtxSpecialRegister, SpirvWord>,
    id_to_reg: HashMap<SpirvWord, PtxSpecialRegister>,
}

impl SpecialRegistersMap {
    fn new() -> Self {
        SpecialRegistersMap {
            reg_to_id: HashMap::new(),
            id_to_reg: HashMap::new(),
        }
    }

    fn get(&self, id: SpirvWord) -> Option<PtxSpecialRegister> {
        self.id_to_reg.get(&id).copied()
    }

    fn get_or_add(&mut self, current_id: &mut SpirvWord, reg: PtxSpecialRegister) -> SpirvWord {
        match self.reg_to_id.entry(reg) {
            hash_map::Entry::Occupied(e) => *e.get(),
            hash_map::Entry::Vacant(e) => {
                let numeric_id = SpirvWord(current_id.0);
                current_id.0 += 1;
                e.insert(numeric_id);
                self.id_to_reg.insert(numeric_id, reg);
                numeric_id
            }
        }
    }
}

struct FnStringIdResolver<'input, 'b> {
    current_id: &'b mut SpirvWord,
    global_variables: &'b HashMap<Cow<'input, str>, SpirvWord>,
    global_type_check: &'b HashMap<u32, Option<(ast::Type, ast::StateSpace, bool)>>,
    special_registers: &'b mut SpecialRegistersMap,
    variables: Vec<HashMap<Cow<'input, str>, SpirvWord>>,
    type_check: HashMap<u32, Option<(ast::Type, ast::StateSpace, bool)>>,
}

impl<'a, 'b> FnStringIdResolver<'a, 'b> {
    fn finish(self) -> NumericIdResolver<'b> {
        NumericIdResolver {
            current_id: self.current_id,
            global_type_check: self.global_type_check,
            type_check: self.type_check,
            special_registers: self.special_registers,
        }
    }

    fn start_block(&mut self) {
        self.variables.push(HashMap::new())
    }

    fn end_block(&mut self) {
        self.variables.pop();
    }

    fn get_id(&mut self, id: &str) -> Result<SpirvWord, TranslateError> {
        for scope in self.variables.iter().rev() {
            match scope.get(id) {
                Some(id) => return Ok(*id),
                None => continue,
            }
        }
        match self.global_variables.get(id) {
            Some(id) => Ok(*id),
            None => {
                let sreg = PtxSpecialRegister::try_parse(id).ok_or_else(error_unknown_symbol)?;
                Ok(self.special_registers.get_or_add(self.current_id, sreg))
            }
        }
    }

    fn add_def(
        &mut self,
        id: &'a str,
        typ: Option<(ast::Type, ast::StateSpace)>,
        is_variable: bool,
    ) -> SpirvWord {
        let numeric_id = *self.current_id;
        self.variables
            .last_mut()
            .unwrap()
            .insert(Cow::Borrowed(id), numeric_id);
        self.type_check.insert(
            numeric_id.0,
            typ.map(|(typ, space)| (typ, space, is_variable)),
        );
        self.current_id.0 += 1;
        numeric_id
    }

    #[must_use]
    fn add_defs(
        &mut self,
        base_id: &'a str,
        count: u32,
        typ: ast::Type,
        state_space: ast::StateSpace,
        is_variable: bool,
    ) -> impl Iterator<Item = SpirvWord> {
        let numeric_id = *self.current_id;
        for i in 0..count {
            self.variables.last_mut().unwrap().insert(
                Cow::Owned(format!("{}{}", base_id, i)),
                SpirvWord(numeric_id.0 + i),
            );
            self.type_check.insert(
                numeric_id.0 + i,
                Some((typ.clone(), state_space, is_variable)),
            );
        }
        self.current_id.0 += count;
        (0..count)
            .into_iter()
            .map(move |i| SpirvWord(i + numeric_id.0))
    }
}

struct NumericIdResolver<'b> {
    current_id: &'b mut SpirvWord,
    global_type_check: &'b HashMap<u32, Option<(ast::Type, ast::StateSpace, bool)>>,
    type_check: HashMap<u32, Option<(ast::Type, ast::StateSpace, bool)>>,
    special_registers: &'b mut SpecialRegistersMap,
}

impl<'b> NumericIdResolver<'b> {
    fn finish(self) -> MutableNumericIdResolver<'b> {
        MutableNumericIdResolver { base: self }
    }

    fn get_typed(
        &self,
        id: SpirvWord,
    ) -> Result<(ast::Type, ast::StateSpace, bool), TranslateError> {
        match self.type_check.get(&id.0) {
            Some(Some(x)) => Ok(x.clone()),
            Some(None) => Err(TranslateError::UntypedSymbol),
            None => match self.special_registers.get(id) {
                Some(x) => Ok((x.get_type(), ast::StateSpace::Sreg, true)),
                None => match self.global_type_check.get(&id.0) {
                    Some(Some(result)) => Ok(result.clone()),
                    Some(None) | None => Err(TranslateError::UntypedSymbol),
                },
            },
        }
    }

    // This is for identifiers which will be emitted later as OpVariable
    // They are candidates for insertion of LoadVar/StoreVar
    fn register_variable(&mut self, typ: ast::Type, state_space: ast::StateSpace) -> SpirvWord {
        let new_id = *self.current_id;
        self.type_check
            .insert(new_id.0, Some((typ, state_space, true)));
        self.current_id.0 += 1;
        new_id
    }

    fn register_intermediate(&mut self, typ: Option<(ast::Type, ast::StateSpace)>) -> SpirvWord {
        let new_id = *self.current_id;
        self.type_check
            .insert(new_id.0, typ.map(|(t, space)| (t, space, false)));
        self.current_id.0 += 1;
        new_id
    }
}

struct MutableNumericIdResolver<'b> {
    base: NumericIdResolver<'b>,
}

impl<'b> MutableNumericIdResolver<'b> {
    fn unmut(self) -> NumericIdResolver<'b> {
        self.base
    }

    fn get_typed(&self, id: SpirvWord) -> Result<(ast::Type, ast::StateSpace), TranslateError> {
        self.base.get_typed(id).map(|(t, space, _)| (t, space))
    }

    fn register_intermediate(&mut self, typ: ast::Type, state_space: ast::StateSpace) -> SpirvWord {
        self.base.register_intermediate(Some((typ, state_space)))
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum TranslateError {
        UnknownSymbol {}
        UntypedSymbol {}
        MismatchedType {}
        Spirv(err: rspirv::dr::Error) {
            from()
            display("{}", err)
            cause(err)
        }
        Unreachable {}
        Todo {}
    }
}

#[cfg(debug_assertions)]
fn error_unreachable() -> TranslateError {
    unreachable!()
}

#[cfg(not(debug_assertions))]
fn error_unreachable() -> TranslateError {
    TranslateError::Unreachable
}

fn error_unknown_symbol() -> TranslateError {
    TranslateError::UnknownSymbol
}

pub struct GlobalFnDeclResolver<'input, 'a> {
    fns: &'a HashMap<SpirvWord, FnSigMapper<'input>>,
}

impl<'input, 'a> GlobalFnDeclResolver<'input, 'a> {
    fn get_fn_sig_resolver(&self, id: SpirvWord) -> Result<&FnSigMapper<'input>, TranslateError> {
        self.fns.get(&id).ok_or_else(error_unknown_symbol)
    }
}

struct FnSigMapper<'input> {
    // true - stays as return argument
    // false - is moved to input argument
    return_param_args: Vec<bool>,
    func_decl: Rc<RefCell<ast::MethodDeclaration<'input, SpirvWord>>>,
}

impl<'input> FnSigMapper<'input> {
    fn remap_to_spirv_repr(mut method: ast::MethodDeclaration<'input, SpirvWord>) -> Self {
        let return_param_args = method
            .return_arguments
            .iter()
            .map(|a| a.state_space != ast::StateSpace::Param)
            .collect::<Vec<_>>();
        let mut new_return_arguments = Vec::new();
        for arg in method.return_arguments.into_iter() {
            if arg.state_space == ast::StateSpace::Param {
                method.input_arguments.push(arg);
            } else {
                new_return_arguments.push(arg);
            }
        }
        method.return_arguments = new_return_arguments;
        FnSigMapper {
            return_param_args,
            func_decl: Rc::new(RefCell::new(method)),
        }
    }

    /*
    fn resolve_in_spirv_repr(
        &self,
        call_inst: ast::CallInst<NormalizedArgParams>,
    ) -> Result<ResolvedCall<NormalizedArgParams>, TranslateError> {
        let func_decl = (*self.func_decl).borrow();
        let mut return_arguments = Vec::new();
        let mut input_arguments = call_inst
            .param_list
            .into_iter()
            .zip(func_decl.input_arguments.iter())
            .map(|(id, var)| (id, var.v_type.clone(), var.state_space))
            .collect::<Vec<_>>();
        let mut func_decl_return_iter = func_decl.return_arguments.iter();
        let mut func_decl_input_iter = func_decl.input_arguments[input_arguments.len()..].iter();
        for (idx, id) in call_inst.ret_params.iter().enumerate() {
            let stays_as_return = match self.return_param_args.get(idx) {
                Some(x) => *x,
                None => return Err(TranslateError::MismatchedType),
            };
            if stays_as_return {
                if let Some(var) = func_decl_return_iter.next() {
                    return_arguments.push((*id, var.v_type.clone(), var.state_space));
                } else {
                    return Err(TranslateError::MismatchedType);
                }
            } else {
                if let Some(var) = func_decl_input_iter.next() {
                    input_arguments.push((
                        ast::Operand::Reg(*id),
                        var.v_type.clone(),
                        var.state_space,
                    ));
                } else {
                    return Err(TranslateError::MismatchedType);
                }
            }
        }
        if return_arguments.len() != func_decl.return_arguments.len()
            || input_arguments.len() != func_decl.input_arguments.len()
        {
            return Err(TranslateError::MismatchedType);
        }
        Ok(ResolvedCall {
            return_arguments,
            input_arguments,
            uniform: call_inst.uniform,
            name: call_inst.func,
        })
    }
     */
}

enum Statement<I, P: ast::Operand> {
    Label(SpirvWord),
    Variable(ast::Variable<P::Ident>),
    Instruction(I),
    // SPIR-V compatible replacement for PTX predicates
    Conditional(BrachCondition),
    LoadVar(LoadVarDetails),
    StoreVar(StoreVarDetails),
    Conversion(ImplicitConversion),
    Constant(ConstantDefinition),
    RetValue(ast::RetData, SpirvWord),
    PtrAccess(PtrAccess<P>),
    RepackVector(RepackVectorDetails),
    FunctionPointer(FunctionPointerDetails),
}

struct BrachCondition {
    predicate: SpirvWord,
    if_true: SpirvWord,
    if_false: SpirvWord,
}
struct LoadVarDetails {
    arg: ast::LdArgs<SpirvWord>,
    typ: ast::Type,
    state_space: ast::StateSpace,
    // (index, vector_width)
    // HACK ALERT
    // For some reason IGC explodes when you try to load from builtin vectors
    // using OpInBoundsAccessChain, the one true way to do it is to
    // OpLoad+OpCompositeExtract
    member_index: Option<(u8, Option<u8>)>,
}

struct StoreVarDetails {
    arg: ast::StArgs<SpirvWord>,
    typ: ast::Type,
    member_index: Option<u8>,
}

#[derive(Clone)]
struct ImplicitConversion {
    src: SpirvWord,
    dst: SpirvWord,
    from_type: ast::Type,
    to_type: ast::Type,
    from_space: ast::StateSpace,
    to_space: ast::StateSpace,
    kind: ConversionKind,
}

#[derive(PartialEq, Clone)]
enum ConversionKind {
    Default,
    // zero-extend/chop/bitcast depending on types
    SignExtend,
    BitToPtr,
    PtrToPtr,
    AddressOf,
}

struct ConstantDefinition {
    pub dst: SpirvWord,
    pub typ: ast::ScalarType,
    pub value: ast::ImmediateValue,
}

pub struct PtrAccess<T> {
    underlying_type: ast::Type,
    state_space: ast::StateSpace,
    dst: SpirvWord,
    ptr_src: SpirvWord,
    offset_src: T,
}

struct RepackVectorDetails {
    is_extract: bool,
    typ: ast::ScalarType,
    packed: SpirvWord,
    unpacked: Vec<SpirvWord>,
    non_default_implicit_conversion: Option<
        fn(
            (ast::StateSpace, &ast::Type),
            (ast::StateSpace, &ast::Type),
        ) -> Result<Option<ConversionKind>, TranslateError>,
    >,
}

struct FunctionPointerDetails {
    dst: SpirvWord,
    src: SpirvWord,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct SpirvWord(spirv::Word);

impl From<spirv::Word> for SpirvWord {
    fn from(value: spirv::Word) -> Self {
        Self(value)
    }
}
impl From<SpirvWord> for spirv::Word {
    fn from(value: SpirvWord) -> Self {
        value.0
    }
}

impl ast::Operand for SpirvWord {
    type Ident = Self;
}

fn pred_map_variable<U, T, F: FnMut(T) -> Result<U, TranslateError>>(
    this: ast::PredAt<T>,
    f: &mut F,
) -> Result<ast::PredAt<U>, TranslateError> {
    let new_label = f(this.label)?;
    Ok(ast::PredAt {
        not: this.not,
        label: new_label,
    })
}

impl<T: ast::Operand, U: ast::Operand, X: FnMut(&str) -> Result<SpirvWord, Err>, Err> ast::VisitorMap<T, U, Err> for X {
    fn visit(
        &mut self,
        args: T,
        type_space: Option<(&ptx_parser::Type, ptx_parser::StateSpace)>,
        is_dst: bool,
    ) -> U {
        todo!()
    }

    fn visit_ident(
        &mut self,
        args: <T as ptx_parser::Operand>::Ident,
        type_space: Option<(&ptx_parser::Type, ptx_parser::StateSpace)>,
        is_dst: bool,
    ) -> <U as ptx_parser::Operand>::Ident {
        todo!()
    }
}

fn op_map_variable<'a, F: FnMut(&str) -> Result<SpirvWord, TranslateError>>(
    this: ast::Instruction<ast::ParsedOperand<&'a str>>,
    f: &mut F,
) -> Result<ast::Instruction<ast::ParsedOperand<SpirvWord>>, TranslateError> {
    ast::visit_map(this , f)
}
