from __future__ import annotations

import argparse
import ast
from pathlib import Path
import time

from frontend.semantic_visitor import SemanticBuilder


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Build custom IR for Python dimension checking."
    )
    parser.add_argument(
        "paths",
        nargs="*",
        default=["examples"],
        help="Python files or directories to analyze",
    )
    return parser.parse_args()


def iter_python_files(paths: list[str]):
    for raw in paths:
        path = Path(raw)
        if path.is_dir():
            yield from sorted(path.glob("*.py"))
        elif path.suffix == ".py":
            yield path


def build_file(path: Path):
    source = path.read_text(encoding="utf-8")
    tree = ast.parse(source, filename=str(path))
    builder = SemanticBuilder(module_name=path.name, file_path=str(path))
    program_ir = builder.build(tree)
    return program_ir


def main() -> None:
    args = parse_args()
    start = time.time()

    for path in iter_python_files(args.paths):
        build_file(path)

    print(f"Successfully generated IR in: {time.time() - start:.4f} seconds")


if __name__ == "__main__":
    main()
