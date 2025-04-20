pub(crate) mod pass;
#[cfg(test)]
mod test;

pub use pass::{TranslateError, to_llvm_module};
pub use pass::emit_llvm::bitcode_to_ir;