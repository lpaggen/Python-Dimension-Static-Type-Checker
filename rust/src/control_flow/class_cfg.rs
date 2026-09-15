use crate::control_flow::graph::Graph;

pub struct ClassCfg<'a> {
    pub graph: Graph<'a>,
    pub scope_id: i64,
}

impl<'a> ClassCfg<'a> {
    pub fn new(scope_id: i64) -> Self {
        Self { 
            graph: Graph::new(),
            scope_id: scope_id,
        }
    }
}
