
pub struct Module {
    version: (u8, u8),
    target: Target
}

pub struct Target {
    arch: String,
    texturing: TexturingMode,
    debug: bool,
    f64_to_f32: bool
}

pub enum TexturingMode {
    Unspecified,
    Unified,
    Independent
}