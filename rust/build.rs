fn main() {
    println!("cargo:rerun-if-changed=../proto/.proto");
    prost_build::compile_protos(&["../proto/.proto"], &["../proto"]).unwrap();
}
