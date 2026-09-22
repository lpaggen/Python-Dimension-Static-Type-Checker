pub mod control_flow;
pub mod diagnostic;
pub mod ir;
pub mod linker;
pub mod pb_decoder;
pub mod solver;
pub mod type_resolver;
pub mod types;

pub mod pb {
    include!(concat!(env!("OUT_DIR"), "/pdc.ir.rs"));
}

use std::{cell::RefCell, rc::Rc};

use control_flow::{
    blockflow::BlockFlow,
    cfg_analysis_engine::{
        analysis_engine::AnalysisEngine, functioncontract_table::FunctionContractTable,
    },
    cfg_table::CfgTable,
};
use diagnostic::diagnostic::Diagnostic;
use ir::program_ir::ProgramIR;
use linker::{
    import_graph::ImportGraph, program_table::ProgramTable, resolution_table::ResolutionTable,
    scope_table::GlobalSymbolTable,
};
use pb_decoder::pb_decoder::PBDecoder;
use type_resolver::type_resolver::TypeResolver;

pub fn analyze_programs(programs: Vec<ProgramIR>) -> Vec<Diagnostic> {
    let mut table = ProgramTable::new();
    table.build_tables(programs);

    let mut symbols = GlobalSymbolTable::new();
    symbols.build(&table);

    let mut import_graph = ImportGraph::new();
    import_graph.build(&table);

    let mut resolutions = ResolutionTable::new();
    resolutions.resolve_imports(&table, &symbols);

    let mut cfg = CfgTable::new();
    cfg.build(&table);

    let contracts = Rc::new(RefCell::new(FunctionContractTable::new()));
    let flow = BlockFlow::new(
        TypeResolver::new(&symbols, &resolutions, Rc::clone(&contracts)),
        &symbols,
        Rc::clone(&contracts),
    );
    let mut engine = AnalysisEngine { flow, contracts };
    let _ = engine.run(&cfg, &table);

    std::mem::take(&mut engine.flow.type_resolver.diagnostics)
}

pub fn analyze_protobuf(bytes: &[u8], source_name: &str) -> Result<Vec<Diagnostic>, String> {
    let program = PBDecoder::decode_bytes(bytes, source_name).map_err(|error| error.to_string())?;
    Ok(analyze_programs(vec![program]))
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn analyze_ir(bytes: &[u8]) -> String {
    match analyze_protobuf(bytes, "playground.py") {
        Ok(diagnostics) => serde_json::to_string(&diagnostics).unwrap_or_else(|error| {
            format!(r#"{{"error":"failed to serialize diagnostics: {error}"}}"#)
        }),
        Err(error) => serde_json::json!({ "error": error }).to_string(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn protobuf_api_returns_shape_diagnostics() {
        let bytes = std::fs::read("../ir_out/ex7.pb").expect("ex7 protobuf should exist");
        let diagnostics = super::analyze_protobuf(&bytes, "example/ex7.py")
            .expect("ex7 protobuf should decode");

        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.message.starts_with("incompatible shapes for matmul:")
                && diagnostic.message.contains("[1, 2] @ [3, 1]")
                && diagnostic.message.contains("2 != 3")
                && !diagnostic.message.contains("pdc_")
                && diagnostic
                    .span
                    .as_ref()
                    .is_some_and(|span| span.lineno == 25)
        }));
    }
}
