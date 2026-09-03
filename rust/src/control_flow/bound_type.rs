use crate::{control_flow::bindingstate::BindingState, types::types::Type};

#[derive(Clone, PartialEq, Debug)]
pub struct TypedBinding {
    pub binding: BindingState,
    pub ty: Type,
}

impl TypedBinding {
    pub fn merge_binding(&self, other: TypedBinding) -> Self {
        Self {
            binding: self.binding.clone().merge_binding(other.binding),
            ty: self.ty.clone().merge(other.ty.clone()),
        }
    }
}
