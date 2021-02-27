use rustc_hash::FxHashMap;
use std::{alloc::Layout, ffi::CString};

#[derive(Clone)]
pub struct VariablesBlock {
    pub variables: FxHashMap<CString, Variable>,
    pub layout: Layout,
}

impl VariablesBlock {
    pub fn empty() -> Self {
        Self {
            variables: FxHashMap::default(),
            layout: Layout::new::<()>(),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Variable {
    pub size: u32,
    pub offset: u32,
    pub default_value: Vec<u8>,
}
