use std::collections::HashMap;

use crate::{control_flow::{block_id::{BlockID, ClassID, FunctionID}, graph::Graph}, ir::stmt::StmtIR};

#[derive(Debug, Clone)]
pub struct ProgramCfg<'a> {
    pub module: Graph<'a>,
    pub functions: HashMap<FunctionID, Graph<'a>>,
    pub classes: HashMap<ClassID, Graph<'a>>,

    pub current_function_id: usize,
    pub current_class_id: usize,
}

impl<'a> ProgramCfg<'a> {
    pub fn new() -> Self {
        Self {
            module: Graph::new(),
            functions: HashMap::new(),
            classes: HashMap::new(),
            current_class_id: 0,
            current_function_id: 0
        }
    }

    pub fn build_program(&mut self, body: &'a [StmtIR]) {
        let mut module = Graph::new();

        module.build(
            self,
            vec![BlockID { id: 0 }],
            body,
            None,
        );

        self.module = module;
    }
}
