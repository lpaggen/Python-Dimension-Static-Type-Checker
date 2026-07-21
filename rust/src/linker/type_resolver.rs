use crate::{
    ir::nodes::AnnotationIR, 
    linker::{program_table::ProgramTable, 
    resolution_table::ResolutionTable, 
    symbol_type_table::SymbolTypeTable}, 
    types::types::Type::{self, None
}};

pub struct TypeResolver<'a> {
    resolutions: &'a ResolutionTable,
    symbol_types: &'a SymbolTypeTable,
}

impl<'a> TypeResolver<'a> {
    pub fn new(
        resolutions: &'a ResolutionTable,
        symbol_types: &'a SymbolTypeTable,
    ) -> Self {
        Self {
            resolutions,
            symbol_types,
        }
    }

    pub fn resolve_types(&self) {
        println!("resolve_types called");
        println!("imports.len() = {}", self.resolutions.imports.len());

        for (symbol_ref, target_ref) in &self.resolutions.imports {
            println!("{:?} {:?}", symbol_ref, target_ref);
        }
    }

    pub fn resolve_annotation(&self, annotation: &AnnotationIR, program_id: i64, scope_id: i64) -> Type {
        println!("{:?}", annotation);
        None
    }
}
