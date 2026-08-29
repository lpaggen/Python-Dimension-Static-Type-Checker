use crate::diagnostic::diagnostic::Diagnostic;
use crate::ir::program_ir::ProgramIR;
use crate::linker::global_scope_table::GlobalSymbolTable;
use crate::linker::import_graph::ImportGraph;
use crate::linker::program_table::ProgramTable;
use crate::linker::resolution_table::ResolutionTable;
use crate::type_resolver::symbol_type_table::SymbolTypeTable;
// use crate::type_resolver::type_resolver::TypeResolver;
use crate::pb_decoder::pb_decoder::PBDecoder;
use crate::control_flow::block_id::BlockID;
use crate::control_flow::cfg::Cfg;

mod diagnostic;
mod ir;
mod linker;
mod pb_decoder;
mod type_resolver;
mod types;
mod control_flow;

pub mod pb {
    include!(concat!(env!("OUT_DIR"), "/pdc.ir.rs"));
}

fn main() -> Result<(), Vec<Diagnostic>> {
    let decoder: PBDecoder = PBDecoder::new("../ir_out/");

    let programs: Vec<ProgramIR> = match decoder.decode_dir() {
        Ok(programs) => programs,
        Err(err) => panic!("{}", err), // should not panic since it depends on Python frontend
    };

    let mut table: ProgramTable = ProgramTable::new();
    table.build_tables(programs);

    let mut symbols = GlobalSymbolTable::new();
    symbols.build(&table);

    let mut graph: ImportGraph = ImportGraph::new();
    graph.build(&table);

    let mut resolved: ResolutionTable = ResolutionTable::new();
    resolved.resolve_imports(&table, &symbols);

    let mut types: SymbolTypeTable = SymbolTypeTable::new();
    types.build(&table, &symbols, &resolved)?;

    let body = &table.by_id.get(&0).unwrap().body;
    println!("{:#?}", body);

    let mut cfg = Cfg::new();

    let exits = cfg.build(
        vec![BlockID { id: 0 }],
        &body,
        None,
    );

    println!("{:?}", cfg.blocks);
    println!("exits: {exits:?}");


    // let mut resolver: TypeResolver = TypeResolver::new(&table, &types);
    // resolver.resolve_types();

    // for (id, program) in table.by_id {
    //     resolver.infer_statements(progr, env, program_id);
    // }

    // println!("{:?}", graph.tarjan_scc());

    Ok(())
}
