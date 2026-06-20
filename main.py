from __future__ import annotations
import argparse
from semantic_visitor import SemanticBuilder
import ast
import os
import time

def main():
    for file in os.listdir("example/"):
        start = time.time()
        path = os.path.join("example", file)

        with open(path, "r", encoding="utf-8") as f:
            source = f.read()

        builder = SemanticBuilder(file, path)
        tree = ast.parse(source, filename=path)

        builder.build(tree)

    end = time.time()
    print(f"Succesfully generated Protobuf IR in: {end - start} seconds!")

if __name__ == "__main__":
    main()
