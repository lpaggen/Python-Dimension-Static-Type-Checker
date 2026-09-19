use crate::{control_flow::block_id::FunctionID, types::types::Type};

// this is used to find contracts, suppose we have
// def foo(a: int, b: int)... AND call foo(5, 5)...
// then we don't need to rebuild constraints for that call if it exists already before
// it's just memoization, if this exact function + args has been called, reuse existing contract
// else send a request to the AnalysisEngine to compute new downstream constraints again
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionSpecializationKey {
    pub function_id: FunctionID,
    pub params: Vec<Type>,
}
