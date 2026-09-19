# Browser playground build

The browser build has two in-page runtimes:

1. Pyodide runs the existing Python `ast` frontend and emits protobuf bytes.
2. `pdc_rust_check_bg.wasm` analyzes those bytes and returns JSON diagnostics.

Build the Rust package from `rust/`:

```sh
cargo build --release --lib --target wasm32-unknown-unknown
wasm-bindgen --target web --out-dir ../browser/pkg \
  target/wasm32-unknown-unknown/release/pdc_rust_check.wasm
```

Build the Python frontend archive from the repository root:

```sh
uv run browser/build_frontend.py
```

The generated `pkg/` directory can be copied into the static website. The
browser must define `globalThis.pdcSolveSmt2(query)` before symbolic constraints
are analyzed. Concrete dimension checks are evaluated directly and do not need
the Z3 bridge.
