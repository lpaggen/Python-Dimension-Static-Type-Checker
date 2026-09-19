use std::collections::HashMap;

use crate::control_flow::{block_id::FunctionID, cfg_analysis_engine::function_contract_key::FunctionSpecializationKey, function_contract::FunctionContract};

// table shared by TypeResolver and BlockFlow
// BlockFlow updates it by insertion
// TypeResolver queries it
// then the engine asks BlockFlow to re-analyze where appropriate
pub struct FunctionContractTable {
    pub by_id: HashMap<FunctionID, FunctionContract>,
    pub specialized: HashMap<FunctionSpecializationKey, FunctionContract>,
}

impl FunctionContractTable {
    pub fn new() -> Self {
        Self { 
            by_id: HashMap::new(),
            specialized: HashMap::new(),
        }
    }
}
