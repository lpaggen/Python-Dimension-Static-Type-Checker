from __future__ import annotations
import argparse
from semantic_visitor import SemanticBuilder
from constraint_solver import ConstraintSolver
from symbol_table import Env
import ast


def main():
    parser = argparse.ArgumentParser(
        description="A simple command-line tool example."
    )
    parser.add_argument(
        "path_to_file",
        help="The file you want to analyze"
    )
    args = parser.parse_args()
    with open(args.path_to_file, "r") as f:
        code = f.read()
    tree = ast.parse(code)
    env = Env()  # same context to be used in all visitors
    builder = SemanticBuilder(env)
    builder.visit(tree)
    c_solver = ConstraintSolver(env, True, False)
    print(c_solver.solve(builder.ir))


if __name__ == "__main__":
    main()