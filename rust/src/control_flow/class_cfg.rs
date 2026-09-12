use crate::control_flow::graph::Graph;

pub struct ClassCfg<'a> {
    pub graph: Graph<'a>,
}

impl<'a> ClassCfg<'a> {
    pub fn new() -> Self {
        Self { 
            graph: Graph::new()
        }
    }
}
