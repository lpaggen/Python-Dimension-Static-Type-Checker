use std::collections::HashMap;

use crate::{
    control_flow::{bindingstate::BindingState, block_id::BlockID}, linker::symbol_ref::SymbolRef, types::types::Type
};

#[derive(Clone)]
pub struct FlowState {
    pub by_ref: HashMap<SymbolRef, BindingState>
}

impl FlowState {
    pub fn bind(&mut self, symbol_ref: &SymbolRef) {
        self.by_ref.insert(*symbol_ref, BindingState::Bound);
    }

    pub fn new() -> Self {
        Self {
            by_ref: HashMap::new(),
        }
    }
}
