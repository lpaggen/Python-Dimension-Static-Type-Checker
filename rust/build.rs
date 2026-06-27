fn main() {
    prost_build::compile_protos(
        &["../proto/.proto"],
        &["../proto"],
    ).unwrap();
}