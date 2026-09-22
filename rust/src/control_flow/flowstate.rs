use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    control_flow::{bindingstate::BindingState, bound_type::TypedBinding},
    linker::symbol_ref::SymbolRef,
    types::types::{GuardedType, Type},
};

#[derive(Clone, PartialEq, Debug)]
pub struct FlowState {
    // FlowState represents what the analysis knows at a certain point, and holds the information needed to lookup bindings across multiple scopes
    pub guard: z3::ast::Bool, // if no guard, simply assume true, mathematically true = None, forgo Option<>
    pub constraints: Vec<z3::ast::Bool>,
    pub scope_id: usize,
    pub by_ref: HashMap<SymbolRef, TypedBinding>,  // TypedBinding holds the BOUND status + type
    pub lexical_parent: Option<Rc<RefCell<FlowState>>>, // since one parent may have multiple children states, each one needs a reference to it
}

impl FlowState {
    pub fn lookup(
        &self, 
        symbol_ref: &SymbolRef,
    ) -> Option<TypedBinding> {
        if let Some(binding) = self.by_ref.get(symbol_ref) {
            return Some(binding.clone());
        }

        self.lexical_parent
            .as_ref()
            .and_then(|parent| parent.borrow().lookup(symbol_ref))
    }

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

    pub fn new(
        scope_id: usize,
        guard: z3::ast::Bool,
        lexical_parent: Option<Rc<RefCell<FlowState>>>,
    ) -> Self {
        Self {
            scope_id,
            guard,
            constraints: Vec::new(),
            by_ref: HashMap::new(),
            lexical_parent,
        }
    }

    pub fn merge<'a>(states: impl IntoIterator<Item = &'a FlowState>) -> FlowState {
        let mut states = states.into_iter();

        // init with false, accumulate new facts as we go
        let first = states
            .next()
            .expect("cannot merge zero flow states");

        let mut merged = FlowState::new(
            first.scope_id,
            z3::ast::Bool::from_bool(false),
            first.lexical_parent.clone(),
        );

        // there may be a better way to store this, currently this is the best i can come up with
        let mut binding_guards: HashMap<SymbolRef, z3::ast::Bool> = HashMap::new();

        // only reachable if any one of the INCOMING edges is reachable
        for state in std::iter::once(first).chain(states) {
            merged.guard = z3::ast::Bool::or(&[&merged.guard, &state.guard]);

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
                            z3::ast::Bool::or(&[
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
