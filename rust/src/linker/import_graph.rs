use std::collections::{HashMap, HashSet};

use crate::{
    linker::program_table::ProgramTable,
};

pub struct ImportGraph {
    pub outgoing: HashMap<i64, HashSet<i64>>,
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub struct ImportRef {
//     pub program_id: i64,
//     pub import_id: i64,
// }

impl ImportGraph {
    pub fn new() -> Self {
        Self {
            outgoing: HashMap::new(),
        }
    }

    pub fn build(&mut self, table: &ProgramTable) {
        for (&importer_id, program) in &table.by_id {
            for import in &program.imports {
                let Some(&imported_id) = table.by_name.get(&import.module_name) else {
                    continue;
                };

                self.outgoing
                    .entry(importer_id)
                    .or_default()
                    .insert(imported_id);
            }
        }
    }

    pub fn imports_of(&self, program_id: i64) -> Option<&HashSet<i64>> {
        self.outgoing.get(&program_id)
    }

}
