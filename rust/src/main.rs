use crate::diagnostic::diagnostic::Diagnostic;
use crate::ir::nodes::program_ir::ProgramIR;
use crate::linker::import_graph::ImportGraph;
use crate::linker::program_table::ProgramTable;
use crate::linker::resolution_table::ResolutionTable;
use crate::linker::symbol_type_table::SymbolTypeTable;
use crate::linker::type_resolver::TypeResolver;
use crate::pb_decoder::pb_decoder::PBDecoder;

mod linker;
mod pb_decoder;

mod diagnostic;

mod types;

mod ir;

pub mod pb {
    include!(concat!(env!("OUT_DIR"), "/pdc.ir.rs"));
}

fn main() -> Result<(), Vec<Diagnostic>> {
    // let bytes = std::fs::read("../ir_out/ex1.pb")?;

    // let program = pb::ProgramIr::decode(bytes.as_slice())?;

    // println!("module: {}", program.module_name);
    // println!("file: {}", program.file_path);
    // println!("scopes: {}", program.scopes.len());
    // println!("symbols: {}", program.symbols.len());
    // println!("imports: {}", program.imports.len());
    // println!("decls: {}", program.decls.len());

    // for decl in &program.decls {
    //     handle_decl(decl);
    // }

    // for node in &program.imports {
    //     println!("{}", node.module_name);
    // }
    let decoder: PBDecoder = PBDecoder::new("../ir_out/");

    let programs: Vec<ProgramIR> = match decoder.decode_dir() {
        Ok(programs) => programs,
        Err(err) => panic!("{}", err), // should not panic since it depends on Python frontend
    };

    // println!("decoded {} programs", programs.len());

    // for program in &programs {
    //     println!("module: {}", program.module_name);
    //     println!("file: {}", program.file_path);
    //     println!("decls: {}", program.decls.len());
    //     println!("imports: {}", program.imports.len());
    // }

    // for program in &programs {
    //     for decl in &program.decls {
    //         match decl {
    //             DeclIR::Binding(binding) => {
    //                 println!("binding: {:?}", binding.kind);
    //             }

    //             DeclIR::Function(function) => {
    //                 println!("function: {}", function.name);
    //             }

    //             DeclIR::Class(class) => {
    //                 println!("class: {}", class.name);
    //             }
    //         }
    //     }
    // }

    let mut table: ProgramTable = ProgramTable::new();
    table.build_tables(programs);

    let mut graph: ImportGraph = ImportGraph::new();
    graph.build(&table);

    let mut resolved: ResolutionTable = ResolutionTable::new();
    resolved.resolve_imports(&table);

    let mut types: SymbolTypeTable = SymbolTypeTable::new();
    types.build(&table)?;

    let resolver: TypeResolver<'_> = TypeResolver::new(&resolved, &types);
    resolver.resolve_types();

    // println!("{:?}", graph.tarjan_scc());

    Ok(())

}
