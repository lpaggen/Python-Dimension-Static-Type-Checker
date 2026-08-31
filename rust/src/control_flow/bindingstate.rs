use crate::types::types::Type;

#[derive(Debug, Clone, PartialEq)]
pub enum BindingState {
    Bound,
    MaybeUnbound,
    Unbound,
}
