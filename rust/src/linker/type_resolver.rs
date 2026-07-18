use crate::{ir::nodes::AnnotationIR, linker::{resolution_table::ResolutionTable, symbol_type_table::SymbolTypeTable}, types::types::Type};

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

    pub fn resolve_annotation(
        &self,
        annotation: &AnnotationIR,
        program_id: i64,
        scope_id: i64,
    ) -> Type {
        todo!()
    }
}
