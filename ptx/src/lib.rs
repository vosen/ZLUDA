pub(crate) mod pass;
#[cfg(test)]
mod test;

pub use pass::to_llvm_module;
pub use pass::to_llvm_module_for_target;
pub use pass::Attributes;
pub use pass::Target;
pub use pass::TranslateError;
