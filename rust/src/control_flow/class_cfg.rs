use crate::control_flow::graph::Graph;

pub struct ClassCfg<'a> {
    pub graph: Graph<'a>,
    pub scope_id: usize,
}

impl<'a> ClassCfg<'a> {
    pub fn new(scope_id: usize) -> Self {
        Self {
            graph: Graph::new(),
            scope_id,
        }
    }
}
