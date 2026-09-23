use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use pdc_rust_check::control_flow::blockflow::BlockFlow;
use pdc_rust_check::control_flow::cfg_analysis_engine::analysis_engine::AnalysisEngine;
use pdc_rust_check::control_flow::cfg_analysis_engine::functioncontract_table::FunctionContractTable;
use pdc_rust_check::control_flow::cfg_table::CfgTable;
use pdc_rust_check::diagnostic::diagnostic::Diagnostic;
use pdc_rust_check::ir::program_ir::ProgramIR;
use pdc_rust_check::linker::import_graph::ImportGraph;
use pdc_rust_check::linker::program_table::ProgramTable;
use pdc_rust_check::linker::resolution_table::ResolutionTable;
use pdc_rust_check::linker::scope_table::GlobalSymbolTable;
use pdc_rust_check::pb_decoder::pb_decoder::PBDecoder;
use pdc_rust_check::type_resolver::type_resolver::TypeResolver;

fn main() -> Result<(), Vec<Diagnostic>> {
    let total_start = Instant::now();

    // let start = Instant::now();
    let decoder: PBDecoder = PBDecoder::new("../ir_out/");

    let programs: Vec<ProgramIR> = match decoder.decode_dir() {
        Ok(programs) => programs,
        Err(err) => panic!("{}", err),
    };

    let validation_diagnostics =
        pdc_rust_check::diagnostic::validation::validate_programs(&programs);
    if !validation_diagnostics.is_empty() {
        return Err(validation_diagnostics);
    }
    // println!("decode:          {:?}", start.elapsed());

    // let start = Instant::now();
    let mut table: ProgramTable = ProgramTable::new();
    table.build_tables(programs);
    // println!("program tables:  {:?}", start.elapsed());

    // let start = Instant::now();
    let mut symbols = GlobalSymbolTable::new();
    symbols.build(&table);
    // println!("symbols:         {:?}", start.elapsed());

    // let start = Instant::now();
    let mut graph: ImportGraph = ImportGraph::new();
    graph.build(&table);
    // println!("import graph:    {:?}", start.elapsed());

    // let start = Instant::now();
    let mut resolved: ResolutionTable = ResolutionTable::new();
    resolved.resolve_imports(&table, &symbols);
    // println!("resolution:      {:?}", start.elapsed());

    // let start = Instant::now();
    // let mut types: SymbolTypeTable = SymbolTypeTable::new();
    // types.build(&table, &symbols, &resolved)?;
    // println!("symbol types:    {:?}", start.elapsed());

    // let start = Instant::now();
    let mut cfg = CfgTable::new();
    cfg.build(&table);
    // println!("cfg:             {:?}", start.elapsed());

    // let start = Instant::now();

    let function_contracts = Rc::new(RefCell::new(FunctionContractTable::new()));

    let flow = BlockFlow::new(
        TypeResolver::new(&symbols, &resolved, Rc::clone(&function_contracts)),
        &symbols,
        Rc::clone(&function_contracts),
    );

    let mut analysis_engine = AnalysisEngine {
        flow,
        contracts: Rc::clone(&function_contracts),
    };

    analysis_engine.run(&cfg, &table);
    // println!("flow analysis:   {:?}", start.elapsed());

    println!("\npipeline took:  {:?}", total_start.elapsed());

    // println!("{:?}", cfg.programs.get(&3));

    let diagnostics = std::mem::take(&mut analysis_engine.flow.type_resolver.diagnostics);
    if diagnostics.is_empty() {
        Ok(())
    } else {
        Err(diagnostics)
    }
}
