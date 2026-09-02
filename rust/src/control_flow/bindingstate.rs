use crate::types::types::Type;

#[derive(Debug, Clone, PartialEq)]
pub enum BindingState {
    Bound,
    MaybeUnbound,
    Unbound,
}

impl BindingState {
    pub fn merge_binding(self, other: BindingState) -> BindingState {
        use BindingState::*;

        match (self, other) {
            (Bound, Bound) => Bound,
            (Unbound, Unbound) => Unbound,

            (MaybeUnbound, _)
            | (_, MaybeUnbound) => MaybeUnbound,

            (Bound, Unbound)
            | (Unbound, Bound) => MaybeUnbound,
        }
    }
}
