use crate::ir::nodes::program_ir::ProgramIR;
use crate::linker::importgraph::ImportGraph;
use crate::pb_decoder::pbdecoder::PBDecoder;

mod linker;
mod pb_decoder;

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

    println!("decoded {} programs", programs.len());

    // for program in &programs {
    //     println!("module: {}", program.module_name);
    //     println!("file: {}", program.file_path);
    //     println!("decls: {}", program.decls.len());
    //     println!("imports: {}", program.imports.len());
    // }

    let mut graph = ImportGraph::new();
    graph.build_import_graph(&programs);

    match graph.imports_of("example/ex1.py") {
        Some(imports) => {
            for imported in imports {
                println!("f1 imports {imported}");
            }
        }
        None => {
            println!("f1 has no recorded imports");
        }
    }

    Ok(())

}
