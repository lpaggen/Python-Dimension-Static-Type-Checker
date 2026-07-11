use prost::Message;

mod linker;
use linker::pbdecoder::PBDecoder;

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
    let decoder = PBDecoder::new("path/to/your/pb/files");

    let programs = decoder.decode_dir()?;

    println!("decoded {} programs", programs.len());

    for program in &programs {
        println!("module: {}", program.module_name);
        println!("file: {}", program.file_path);
        println!("decls: {}", program.decls.len());
        println!("imports: {}", program.imports.len());
    }

    Ok(())

}

// fn handle_decl(decl: &pb::DeclIr) {
//     match &decl.kind {
//         Some(pb::decl_ir::Kind::Binding(binding)) => {
//             handle_binding(binding);
//         }

//         Some(pb::decl_ir::Kind::Function(function)) => {
//             handle_function(function);
//         }

//         Some(pb::decl_ir::Kind::ClassDecl(class_decl)) => {
//             handle_class(class_decl);
//         }

//         None => {
//             eprintln!("empty DeclIR");
//         }
//     }
// }

// fn handle_binding(binding: &pb::BindingIr) {
//     println!("binding id: {}", binding.id);
//     println!("target symbol id: {}", binding.target_id);

//     if let Some(value) = &binding.value {
//         handle_expr(value);
//     }
// }

// fn handle_function(function: &pb::FunctionIr) {
//     println!("function: {}", function.name);
//     println!("params: {}", function.params.len());
//     println!("body stmts: {}", function.body.len());

//     for stmt in &function.body {
//         handle_stmt(stmt);
//     }
// }

// fn handle_class(class_decl: &pb::ClassIr) {
//     println!("class: {}", class_decl.name);
//     println!("body stmts: {}", class_decl.body.len());

//     for stmt in &class_decl.body {
//         handle_stmt(stmt);
//     }
// }

// fn handle_stmt(stmt: &pb::StmtIr) {
//     match &stmt.kind {
//         Some(pb::stmt_ir::Kind::Binding(binding)) => {
//             handle_binding(binding);
//         }

//         Some(pb::stmt_ir::Kind::AugAssign(aug)) => {
//             if let Some(target) = &aug.target {
//                 handle_expr(target);
//             }

//             if let Some(value) = &aug.value {
//                 handle_expr(value);
//             }
//         }

//         Some(pb::stmt_ir::Kind::ReturnStmt(ret)) => {
//             if let Some(value) = &ret.value {
//                 handle_expr(value);
//             }
//         }

//         Some(pb::stmt_ir::Kind::ExprStmt(expr_stmt)) => {
//             if let Some(value) = &expr_stmt.value {
//                 handle_expr(value);
//             }
//         }

//         Some(pb::stmt_ir::Kind::IfStmt(if_stmt)) => {
//             if let Some(test) = &if_stmt.test {
//                 handle_expr(test);
//             }

//             for stmt in &if_stmt.body {
//                 handle_stmt(stmt);
//             }

//             for stmt in &if_stmt.orelse {
//                 handle_stmt(stmt);
//             }
//         }

//         Some(pb::stmt_ir::Kind::ForLoop(for_loop)) => {
//             if let Some(target) = &for_loop.target {
//                 handle_expr(target);
//             }

//             if let Some(iter) = &for_loop.iter {
//                 handle_expr(iter);
//             }

//             for stmt in &for_loop.body {
//                 handle_stmt(stmt);
//             }

//             for stmt in &for_loop.orelse {
//                 handle_stmt(stmt);
//             }
//         }

//         Some(pb::stmt_ir::Kind::WhileLoop(while_loop)) => {
//             if let Some(test) = &while_loop.test {
//                 handle_expr(test);
//             }

//             for stmt in &while_loop.body {
//                 handle_stmt(stmt);
//             }

//             for stmt in &while_loop.orelse {
//                 handle_stmt(stmt);
//             }
//         }

//         Some(pb::stmt_ir::Kind::ImportStmt(import_stmt)) => {
//             println!("import: {}", import_stmt.module_name);
//         }

//         None => {
//             eprintln!("empty StmtIR");
//         }
//     }
// }

// fn handle_expr(expr: &pb::ExprIr) {
//     match &expr.kind {
//         Some(pb::expr_ir::Kind::Identifier(identifier)) => {
//             println!("identifier: {}", identifier.name);
//         }

//         Some(pb::expr_ir::Kind::Integer(integer)) => {
//             println!("integer: {}", integer.value);
//         }

//         Some(pb::expr_ir::Kind::FloatLit(float_lit)) => {
//             println!("float: {}", float_lit.value);
//         }

//         Some(pb::expr_ir::Kind::StringLit(string_lit)) => {
//             println!("string: {:?}", string_lit.value);
//         }

//         Some(pb::expr_ir::Kind::BoolLit(bool_lit)) => {
//             println!("bool: {}", bool_lit.value);
//         }

//         Some(pb::expr_ir::Kind::NoneLit(_none_lit)) => {
//             println!("None");
//         }

//         Some(pb::expr_ir::Kind::List(list)) => {
//             for element in &list.elements {
//                 handle_expr(element);
//             }
//         }

//         Some(pb::expr_ir::Kind::Tuple(tuple)) => {
//             for element in &tuple.elements {
//                 handle_expr(element);
//             }
//         }

//         Some(pb::expr_ir::Kind::Call(call)) => {
//             if let Some(callee) = &call.callee {
//                 handle_expr(callee);
//             }

//             for arg in &call.args {
//                 handle_expr(arg);
//             }

//             for kwarg in &call.kwargs {
//                 if let Some(value) = &kwarg.value {
//                     handle_expr(value);
//                 }
//             }
//         }

//         Some(pb::expr_ir::Kind::Attribute(attribute)) => {
//             if let Some(base) = &attribute.base {
//                 handle_expr(base);
//             }

//             println!("attr: {}", attribute.attr);
//         }

//         Some(pb::expr_ir::Kind::Binop(binop)) => {
//             if let Some(left) = &binop.left {
//                 handle_expr(left);
//             }

//             if let Some(right) = &binop.right {
//                 handle_expr(right);
//             }
//         }

//         Some(pb::expr_ir::Kind::Unaryop(unaryop)) => {
//             if let Some(operand) = &unaryop.operand {
//                 handle_expr(operand);
//             }
//         }

//         Some(pb::expr_ir::Kind::Boolop(boolop)) => {
//             for value in &boolop.values {
//                 handle_expr(value);
//             }
//         }

//         Some(pb::expr_ir::Kind::Compare(compare)) => {
//             if let Some(left) = &compare.left {
//                 handle_expr(left);
//             }

//             for comparator in &compare.comparators {
//                 handle_expr(comparator);
//             }
//         }

//         Some(pb::expr_ir::Kind::Subscript(subscript)) => {
//             if let Some(target) = &subscript.target {
//                 handle_expr(target);
//             }

//             if let Some(sub) = &subscript.subscript {
//                 handle_expr(sub);
//             }
//         }

//         Some(pb::expr_ir::Kind::Slice(slice)) => {
//             if let Some(lower) = &slice.lower {
//                 handle_expr(lower);
//             }

//             if let Some(upper) = &slice.upper {
//                 handle_expr(upper);
//             }

//             if let Some(step) = &slice.step {
//                 handle_expr(step);
//             }
//         }

//         None => {
//             eprintln!("empty ExprIR");
//         }
//     }
// }
