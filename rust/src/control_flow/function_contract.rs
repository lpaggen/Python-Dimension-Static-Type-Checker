use crate::types::types::{GuardedType, Type};

#[derive(Debug, Clone)]
pub struct GuardedReturn {
    pub guard: z3::ast::Bool,
    pub ty: Type,
    pub constraints: Vec<z3::ast::Bool>,
}

#[derive(Debug, Clone)]
pub struct FunctionContract {
    pub params: Vec<Type>,
    pub declared_return_type: Type,
    pub returns: Vec<GuardedReturn>,
}

impl FunctionContract {
    pub fn new() -> Self {
        Self {
            params: Vec::new(),
            declared_return_type: Type:: Unknown,
            returns: Vec::new(),
        }
    }
}
