use crate::{
    ir::nodes::AnnotationIR, 
    linker::{ 
        resolution_table::ResolutionTable, 
        symbol_type_table::SymbolTypeTable
    }, 
        types::types::Type::{
            self, 
            None
    }
};

use crate::linker::resolved_target::ResolvedTarget;

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
        for (symbol_ref, symbol_type) in &self.symbol_types.by_ref {
            let target_ref = match self.resolutions.imports.get(symbol_ref) {
                Some(imported_ref) => imported_ref,
                _ => &ResolvedTarget::Local(*symbol_ref),  // can resolve to itself if not an import
            };
            // let resolved_type = self.resolve_annotation(annotation);
        }
    }

    pub fn resolve_annotation(&self, annotation: &AnnotationIR) -> Type {
        println!("{:?}", annotation);
        None
    }
}
