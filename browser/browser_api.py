from main import build_source


def build_ir(source: str, filename: str = "playground.py") -> bytes:
    """Parse editor source and return the protobuf consumed by Rust/WASM."""
    return build_source(source, filename).to_proto().SerializeToString()
