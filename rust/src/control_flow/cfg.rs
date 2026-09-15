use std::collections::HashMap;

use crate::{control_flow::{block_id::{BlockID, ClassID, FunctionID}, class_cfg::ClassCfg, function_cfg::FunctionCfg, graph::Graph, module_cfg::ModuleCfg}, ir::stmt::StmtIR};



pub struct Cfg<'a> {
    pub module: ModuleCfg<'a>,
    pub functions: HashMap<FunctionID, FunctionCfg<'a>>,
    pub classes: HashMap<ClassID, ClassCfg<'a>>,

    pub current_function_id: usize,
    pub current_class_id: usize,
    pub program_id: i64,  // copy of ProgramTable's own ID, needed for SymbolRef creation
}

impl<'a> Cfg<'a> {
    pub fn new(id: i64) -> Self {
        Self {
            module: ModuleCfg { 
                graph: Graph::new(),
                scope_id: 0 // ?
            },
            functions: HashMap::new(),
            classes: HashMap::new(),
            current_class_id: 0,
            current_function_id: 0,
            program_id: id,
        }
    }

    pub fn build_program(&mut self, body: &'a [StmtIR]) {
        let mut module_graph = Graph::new();

        module_graph.build(
            self,
            vec![BlockID { id: 0 }],
            body,
            None,
        );

        self.module = ModuleCfg {
            graph: module_graph,
            scope_id: 0,  // ? 
        };
    }
}
