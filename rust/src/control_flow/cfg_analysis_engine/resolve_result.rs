use crate::{control_flow::function_analysis_request::FunctionAnalysisRequest, types::types::Type};

// this is used so that we can "bubble up" during type resolving
// if it's not a function, no problem, if it is a function,
// then we need to re-analyze the function block with the supplied arguments
// then proceed downstream
// the error makes this work naturally, if it's a function, error -> analysis
pub type ResolveResult = Result<Type, FunctionAnalysisRequest>;
