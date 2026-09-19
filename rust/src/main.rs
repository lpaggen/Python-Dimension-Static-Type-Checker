use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use crate::control_flow::blockflow::BlockFlow;
use crate::control_flow::cfg_analysis_engine::analysis_engine::AnalysisEngine;
use crate::control_flow::cfg_analysis_engine::functioncontract_table::FunctionContractTable;
use crate::control_flow::cfg_table::CfgTable;
use crate::diagnostic::diagnostic::Diagnostic;
use crate::ir::program_ir::ProgramIR;
use crate::linker::import_graph::ImportGraph;
use crate::linker::program_table::ProgramTable;
use crate::linker::resolution_table::ResolutionTable;
use crate::linker::scope_table::GlobalSymbolTable;
use crate::pb_decoder::pb_decoder::PBDecoder;
use crate::type_resolver::type_resolver::TypeResolver;

mod control_flow;
mod diagnostic;
mod ir;
mod linker;
mod pb_decoder;
mod type_resolver;
mod types;

pub mod pb {
    include!(concat!(env!("OUT_DIR"), "/pdc.ir.rs"));
}

fn main() -> Result<(), Vec<Diagnostic>> {
    let total_start = Instant::now();

    let start = Instant::now();
    let decoder: PBDecoder = PBDecoder::new("../ir_out/");

    let programs: Vec<ProgramIR> = match decoder.decode_dir() {
        Ok(programs) => programs,
        Err(err) => panic!("{}", err),
    };
    println!("decode:          {:?}", start.elapsed());

    let start = Instant::now();
    let mut table: ProgramTable = ProgramTable::new();
    table.build_tables(programs);
    println!("program tables:  {:?}", start.elapsed());

    let start = Instant::now();
    let mut symbols = GlobalSymbolTable::new();
    symbols.build(&table);
    println!("symbols:         {:?}", start.elapsed());

    let start = Instant::now();
    let mut graph: ImportGraph = ImportGraph::new();
    graph.build(&table);
    println!("import graph:    {:?}", start.elapsed());

    let start = Instant::now();
    let mut resolved: ResolutionTable = ResolutionTable::new();
    resolved.resolve_imports(&table, &symbols);
    println!("resolution:      {:?}", start.elapsed());

    // let start = Instant::now();
    // let mut types: SymbolTypeTable = SymbolTypeTable::new();
    // types.build(&table, &symbols, &resolved)?;
    // println!("symbol types:    {:?}", start.elapsed());

    let start = Instant::now();
    let mut cfg = CfgTable::new();
    cfg.build(&table);
    println!("cfg:             {:?}", start.elapsed());

    let start = Instant::now();

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
    println!("flow analysis:   {:?}", start.elapsed());

    println!("total pipeline:  {:?}", total_start.elapsed());

    // println!("{:?}", cfg.programs.get(&3));

    Ok(())
}
