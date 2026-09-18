use std::collections::HashMap;

use crate::control_flow::{block_id::FunctionID, function_contract::FunctionContract};

// table shared by TypeResolver and BlockFlow
// BlockFlow updates it by insertion
// TypeResolver queries it
pub struct FunctionContractTable {
    pub by_id: HashMap<FunctionID, FunctionContract>,
}

impl FunctionContractTable {
    pub fn new() -> Self {
        Self { 
            by_id: HashMap::new() 
        }
    }
}
