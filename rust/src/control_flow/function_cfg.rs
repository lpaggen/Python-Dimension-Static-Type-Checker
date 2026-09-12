use crate::{control_flow::graph::Graph, ir::{arg::ArgIR, expr::ExprIR, nodes::TypeParamIR}};

pub struct FunctionCfg<'a> {
    pub graph: Graph<'a>,
    pub params: Vec<ArgIR>,
    pub returns: Option<ExprIR>
}

impl<'a> FunctionCfg<'a> {
    pub fn new(params: Vec<ArgIR>, returns: Option<ExprIR>) -> Self {
        Self {
            graph: Graph::new(),
            params: params,
            returns: returns,
        }
    }
}
