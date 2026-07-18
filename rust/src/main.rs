use crate::ir::nodes::{BindingIR, DeclIR};
use crate::ir::nodes::program_ir::ProgramIR;
use crate::linker::import_graph::ImportGraph;
use crate::linker::program_table::ProgramTable;
use crate::linker::resolution_table::ResolutionTable;
use crate::pb_decoder::pb_decoder::PBDecoder;

mod linker;
mod pb_decoder;

mod diagnostic;

mod types;

mod ir;

pub mod pb {
    include!(concat!(env!("OUT_DIR"), "/pdc.ir.rs"));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let programs: Vec<ProgramIR> = decoder.decode_dir()?;

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

    let mut table = ProgramTable::new();
    table.build_tables(programs);

    let mut graph = ImportGraph::new();
    graph.build(&table);

    let mut resolved = ResolutionTable::new();
    resolved.resolve_imports(&table);

    for (&program_id, program) in &table.by_id {
        println!("{} {} imports:", program_id, program.module_name);
        if let Some(imported_ids) = graph.imports_of(program_id) {
            for &import in imported_ids {
                println!("   {}", import)
            }
        }
    }

    println!("{:?}", graph.tarjan_scc());

    Ok(())

}
