use prost::Message;
use std::env;
use std::fs;

pub mod pdc {
    pub mod ir {
        include!(concat!(env!("OUT_DIR"), "/pdc.ir.rs"));
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| "../ir_out/example.pb".to_string());

    let bytes = fs::read(&path)?;
    let program = pdc::ir::ProgramIr::decode(bytes.as_slice())?;

    println!("Decoded OK!");
    println!("module_name: {}", program.module_name);
    println!("file_path: {}", program.file_path);
    println!("scopes: {}", program.scopes.len());
    println!("symbols: {}", program.symbols.len());
    println!("imports: {}", program.imports.len());
    println!("decls: {}", program.decls.len());

    Ok(())
}