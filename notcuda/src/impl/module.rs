use ptx;

pub struct Module {
    spirv_code: Vec<u32>,
    compiled_code: Vec<Option<Vec<u8>>>, // size as big as the number of devices
}

pub enum ModuleCompileError<'a> {
    Parse(
        Vec<ptx::ast::PtxError>,
        Option<ptx::ParseError<usize, ptx::Token<'a>, &'a str>>,
    ),
    Compile(ptx::SpirvError),
}

impl<'a> ModuleCompileError<'a> {
    pub fn get_build_log(&self) {}
}

impl<'a> From<ptx::SpirvError> for ModuleCompileError<'a> {
    fn from(err: ptx::SpirvError) -> Self {
        ModuleCompileError::Compile(err)
    }
}

impl Module {
    pub fn compile(ptx_text: &str, devices: usize) -> Result<Self, ModuleCompileError> {
        let mut errors = Vec::new();
        let ast = ptx::ModuleParser::new().parse(&mut errors, ptx_text);
        let ast = match ast {
            Err(e) => return Err(ModuleCompileError::Parse(errors, Some(e))),
            Ok(_) if errors.len() > 0 => return Err(ModuleCompileError::Parse(errors, None)),
            Ok(ast) => ast,
        };
        let spirv = ptx::to_spirv(ast)?;
        Ok(Self {
            spirv_code: spirv,
            compiled_code: vec![None; devices],
        })
    }
}
