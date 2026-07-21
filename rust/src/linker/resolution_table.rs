use crate::linker::{program_table::ProgramTable, resolved_target::ResolvedTarget::{self}, symbol_ref::SymbolRef, global_scope_table::GlobalSymbolTable};

use std::{collections::HashMap};

pub struct ResolutionTable {
    pub imports: HashMap<SymbolRef, ResolvedTarget>,
    // pub diagnostics: Vec<String>, // TODO make new class Diagnostic with SpanIR for proper debugging
}


impl ResolutionTable {
    pub fn new() -> Self {
        Self {
            imports: HashMap::new(),
        }
    }

    pub fn resolve_imports(&mut self, programs: &ProgramTable) {
        // let mut resolutions = ResolutionTable {
        //     imports: HashMap::new(),
        // };

        for (&program_id, program) in &programs.by_id {
            for import in &program.imports {
                let symbol_ref = SymbolRef {
                    program_id,
                    symbol_id: import.local_symbol_id,
                };

                let imported_name = match &import.imported_name {
                    Some(name) => name.as_str(),
                    None => import.module_name.as_str(),
                };

                // !! might be a problem if the import goes to external because it doesn't exist in local files due to a typo
                let target = match programs.get_by_name(&import.module_name) { // if in our local project files, Local, else i.e. torch -> Ext
                    Some(_target) => {
                        let target_program_id = *programs.by_name.get(&import.module_name).unwrap();  // cannot fail
                        let target_program = programs.get_by_id(target_program_id).unwrap();
                        let target_global_symbol_lookup = GlobalSymbolTable::build(target_program_id, target_program);
                        let target_symbol = target_global_symbol_lookup.by_name.get(imported_name);
                        ResolvedTarget::Local(*target_symbol.unwrap())
                }
                    None => ResolvedTarget::External {
                        module: import.module_name.clone(),
                        name: imported_name.to_string(),
                    }
                };
                self.imports.insert(symbol_ref, target);
            }
        }
    }
}
