use crate::control_flow::graph::Graph;

pub struct ModuleCfg<'a> {
    pub graph: Graph<'a>,
    pub scope_id: i64,
}
