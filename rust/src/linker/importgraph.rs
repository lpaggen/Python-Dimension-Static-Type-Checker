use std::collections::{HashMap};
use crate::ir::nodes::{ProgramIR, import_ir::ImportIR};

pub struct ImportGraph<'programs> {
    pub outgoing: HashMap<&'programs str, Vec<&'programs ImportIR>>,
}

impl<'programs> ImportGraph<'programs> {
    pub fn new() -> Self {Self {outgoing: HashMap::new()}}

    pub fn add_import(&mut self, importer: &'programs str, imported: &'programs ImportIR) {
        self.outgoing
            .entry(importer)
            .or_default()
            .push(imported);
    }

    pub fn imports_of(&self, module: &str) -> Option<&[&'programs ImportIR]> {
        return self.outgoing.get(module).map(Vec::as_slice);
    }

    pub fn build_import_graph(&mut self, programs: &'programs [ProgramIR]) {
        for program in programs {
            for import in &program.imports {
                self.add_import(&program.module_name.as_str(), import);
            }
        }
    }
}
