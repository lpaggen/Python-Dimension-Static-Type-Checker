use crate::types::types::Type;

#[derive(Debug, Clone)]
pub struct CallBinding {
    pub symbol_id: usize,
    pub ty: Type,
}
