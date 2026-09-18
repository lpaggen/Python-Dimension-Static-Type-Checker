use crate::types::types::Type;

#[derive(Debug, Clone)]
pub struct CallBinding {
    pub symbol_id: i64,
    pub ty: Type,
}
