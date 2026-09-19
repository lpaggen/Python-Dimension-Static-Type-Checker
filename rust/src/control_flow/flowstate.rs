use std::collections::HashMap;

use crate::{
    control_flow::{bindingstate::BindingState, bound_type::TypedBinding},
    linker::symbol_ref::SymbolRef,
    solver::BoolExpr,
    types::types::{GuardedType, Type},
};

#[derive(Clone, PartialEq, Debug)]
pub struct FlowState {
    pub guard: BoolExpr, // if no guard, simply assume true, mathematically true = None, forgo Option<>
    pub by_ref: HashMap<SymbolRef, TypedBinding>,
    pub constraints: Vec<BoolExpr>,
}

impl FlowState {
    pub fn bind(&mut self, symbol_ref: &SymbolRef, ty: Type) {
        self.by_ref.insert(
            *symbol_ref,
            TypedBinding {
                binding: BindingState::Bound,
                ty,
            },
        );
    }

    // very practical use for this, ignore binding status first, only care for type, update status as we go
    pub fn register_unbound(&mut self, symbol_ref: &SymbolRef) {
        self.by_ref.insert(
            *symbol_ref,
            TypedBinding {
                binding: BindingState::Unbound,
                ty: Type::Unknown,
            },
        );
    }

    pub fn new(guard: BoolExpr) -> Self {
        Self {
            by_ref: HashMap::new(),
            constraints: Vec::new(),
            guard,
        }
    }

    pub fn merge<'a>(states: impl IntoIterator<Item = &'a FlowState>) -> FlowState {
        // init with false, accumulate new facts as we go
        let mut merged = FlowState::new(BoolExpr::from_bool(false));

        // there may be a better way to store this, currently this is the best i can come up with
        let mut binding_guards: HashMap<SymbolRef, BoolExpr> = HashMap::new();

        // only reachable if any one of the INCOMING edges is reachable
        for state in states {
            merged.guard = BoolExpr::or(&[&merged.guard, &state.guard]);

            for (id, binding) in &state.by_ref {
                match merged.by_ref.get_mut(id) {
                    // either is exists, we want to update its binding
                    // TODO check soundness of this system
                    Some(existing) => {
                        // figure out how to stop cloning
                        // *existing = existing.merge_binding(binding.clone());

                        // and merge the guards
                        let previous_guard = binding_guards.get(id).unwrap().clone();

                        existing.binding = existing
                            .binding
                            .clone()
                            .merge_binding(binding.binding.clone());

                        if existing.ty != binding.ty {
                            match &mut existing.ty {
                                Type::FlowUnion(alternatives) => {
                                    alternatives.push(GuardedType {
                                        guard: state.guard.clone(),
                                        ty: binding.ty.clone(),
                                    });
                                }

                                _ => {
                                    let previous_ty = // TODO check if this even works 
                                        std::mem::replace(&mut existing.ty, Type::Unknown);

                                    existing.ty = Type::FlowUnion(vec![
                                        GuardedType {
                                            guard: previous_guard.clone(),
                                            ty: previous_ty,
                                        },
                                        GuardedType {
                                            guard: state.guard.clone(),
                                            ty: binding.ty.clone(),
                                        },
                                    ]);
                                }
                            }
                        }
                        binding_guards.insert(
                            *id,
                            BoolExpr::or(&[
                                &previous_guard, // Fixed: no change needed here, but ensure it's properly cloned above
                                &state.guard,
                            ]),
                        );
                    }

                    // or it doesn't exist yet, just insert
                    None => {
                        merged.by_ref.insert(*id, binding.clone());
                        binding_guards.insert(*id, state.guard.clone());
                    }
                }
            }
        }

        merged
    }
}
