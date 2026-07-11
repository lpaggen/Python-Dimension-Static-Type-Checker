use std::collections::{HashMap, HashSet};

use crate::ir::nodes::ProgramIR;

pub struct ImportGraph {
    pub outgoing: HashMap<String, HashSet<String>>,
}

// TODO fix, this isn't ready for use in final pipeline yet
impl ImportGraph {
    pub fn new() -> Self {
        Self {
            outgoing: HashMap::new(),
        }
    }

    pub fn add_import(&mut self, importer: &str, imported: &str) {
        self.outgoing
            .entry(importer.to_owned())
            .or_default()
            .insert(imported.to_owned());
    }

    pub fn imports_of(&self, module: &str) -> Option<&HashSet<String>> {
        self.outgoing.get(module)
    }

    pub fn build_import_graph(&mut self, programs: &[ProgramIR]) {
        for program in programs {
            for import in &program.imports {
                let import_name = if import.alias.is_some() { import.alias.as_deref() } else { import.imported_name.as_deref() };

                match import_name {
                    Some(import_name) => {
                        self.add_import(
                            program.module_name.as_str(),
                            import_name,
                        );
                    }
                    None => {}
                }
            }
        }
    }
}