pub(crate) mod pass;
#[cfg(test)]
mod test;

pub use pass::to_llvm_module;
pub use pass::emit_llvm::bitcode_to_ir;
pub use pass::Attributes;