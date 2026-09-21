use crate::{
    control_flow::graph::Graph,
    ir::{arg::ArgIR, expr::ExprIR},
};

pub struct FunctionCfg<'a> {
    pub graph: Graph<'a>,
    pub params: Vec<ArgIR>,
    pub returns: Option<ExprIR>,
    pub scope_id: usize,
}

impl<'a> FunctionCfg<'a> {
    pub fn new(params: Vec<ArgIR>, returns: Option<ExprIR>, scope_id: usize) -> Self {
        Self {
            graph: Graph::new(),
            params,
            returns,
            scope_id,
        }
    }
}
