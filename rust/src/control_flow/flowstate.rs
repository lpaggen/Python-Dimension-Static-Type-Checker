use std::{collections::HashMap, hash::Hash};

use crate::{
    control_flow::{bindingstate::BindingState, block_id::BlockID}, linker::symbol_ref::SymbolRef, types::types::Type
};

#[derive(Clone, PartialEq)]
pub struct FlowState {
    pub by_ref: HashMap<SymbolRef, BindingState>
}

impl FlowState {
    pub fn bind(&mut self, symbol_ref: &SymbolRef) {
        self.by_ref.insert(*symbol_ref, BindingState::Bound);
    }

    // very practical use for this, ignore binding status first, only care for type, update status as we go
    pub fn register_unbound(&mut self, symbol_ref: &SymbolRef) {
        self.by_ref.insert(*symbol_ref, BindingState::Unbound);
    }

    pub fn new() -> Self {
        Self {
            by_ref: HashMap::new(),
        }
    }

    pub fn merge<'a>(states: impl IntoIterator<Item = &'a FlowState>) -> FlowState {
        let mut merged = FlowState::new();

        for state in states {
            for (id, binding) in &state.by_ref {
                match merged.by_ref.get_mut(&id) {

                    // either is exists, we want to update its binding
                    Some(existing) => {
                        // figure out how to stop cloning 
                        *existing = existing.clone().merge_binding(binding.clone());  // calling BindingState's merge !!
                    },

                    // or it doesn't exist yet, just insert
                    None => {
                        merged.by_ref.insert(*id, binding.clone());
                    }
                }
            }
        }

        merged
    }
}
