use crate::{control_flow::bindingstate::BindingState, types::types::Type};

#[derive(Clone, PartialEq)]
pub struct TypedBinding {
    pub binding: BindingState,
    pub ty: Type,
}

impl TypedBinding {
    pub fn merge_binding(&self, other: BindingState) -> Self {
        Self {
            binding: self.binding.clone().merge_binding(other),
            ty: self.ty.clone(),
        }
    }
}
