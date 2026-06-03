from __future__ import annotations
import argparse
from semantic_visitor import SemanticBuilder
from constraint_solver import ConstraintSolver
from symbol_table import Env
import ast
from type_resolver import TypeResolver
import os
from linker import ImportCollector, Linker


def main():
    parser = argparse.ArgumentParser(
        description="Validate a Python project."
    )

    parser.add_argument(
        "path_to_dir",
        help="Project directory"
    )

    args = parser.parse_args()

    project_scopes = {}
    project_imports = {}

    for file in os.listdir(args.path_to_dir):
        if not file.endswith(".py"):
            continue

        path = os.path.join(args.path_to_dir, file)

        with open(path, "r") as f:
            code = f.read()

        env = Env()

        tree = ast.parse(code)

        module_dependency_resolver = ImportCollector(file)
        module_dependency_resolver.visit(tree)
        project_imports[file] = module_dependency_resolver.imports

        builder = SemanticBuilder(env)
        builder.visit(tree)
        # print(builder.env.dump())

        project_scopes[file] = builder.env

        type_resolver = TypeResolver(
            builder.env.unresolved,
            env
        )


        # type_resolver.resolve_types()

        # c_solver = ConstraintSolver(env)

        # project_scopes[file] = scopes
        
    linker = Linker(project_scopes, project_imports)
    linker.static_link_files()

if __name__ == "__main__":
    main()
