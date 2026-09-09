use std::collections::HashMap;

use crate::{
    control_flow::{bindingstate::BindingState, bound_type::TypedBinding}, linker::symbol_ref::SymbolRef, types::types::Type
};

#[derive(Clone, PartialEq, Debug)]
pub struct FlowState {
    pub guard: z3::ast::Bool,  // if no guard, simply assume true, mathematically true = None, forgo Option<>
    pub by_ref: HashMap<SymbolRef, TypedBinding>,
    pub constraints: Vec<z3::ast::Bool>,
}

impl FlowState {
    pub fn bind(&mut self, symbol_ref: &SymbolRef, ty: Type) {
        self.by_ref.insert(*symbol_ref, TypedBinding { 
            binding: BindingState::Bound, 
            ty: ty 
        });
    }

    // very practical use for this, ignore binding status first, only care for type, update status as we go
    pub fn register_unbound(&mut self, symbol_ref: &SymbolRef) {
        self.by_ref.insert(*symbol_ref, TypedBinding { binding: BindingState::Unbound, ty: Type::Unknown });
    }

    pub fn new(guard: z3::ast::Bool) -> Self {
        Self {
            by_ref: HashMap::new(),
            constraints: Vec::new(),
            guard
        }
    }

    pub fn merge<'a>(states: impl IntoIterator<Item = &'a FlowState>) -> FlowState {
        // init with false, accumulate new facts as we go
        let mut merged = FlowState::new(z3::ast::Bool::from_bool(false));

        // only reachable if any one of the INCOMING edges is reachable
        for state in states {
            merged.guard = z3::ast::Bool::or(&[
                &merged.guard,
                &state.guard,
            ]);

            for (id, binding) in &state.by_ref {
                match merged.by_ref.get_mut(&id) {

                    // either is exists, we want to update its binding
                    Some(existing) => {
                        // figure out how to stop cloning 
                        *existing = existing.merge_binding(binding.clone());
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
